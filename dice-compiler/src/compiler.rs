use crate::{
    assembler::Assembler,
    compiler_stack::{CompilerContext, CompilerKind, CompilerStack},
    scope_stack::State,
    visitor::NodeVisitor,
};
use dice_core::{
    bytecode::Bytecode,
    error::{Error, ResultExt as _},
    protocol::{module::EXPORT, ProtocolSymbol},
    source::{Source, SourceKind},
    span::Span,
};
use dice_syntax::{Parser, SyntaxTree};

pub struct Compiler {
    pub(crate) syntax_tree: SyntaxTree,
    pub(crate) compiler_stack: CompilerStack,
    pub(crate) source: Source,
}

impl Compiler {
    pub fn compile_source(source: Source) -> Result<Bytecode, Error> {
        let syntax_tree = Parser::new(&source).parse()?;
        let kind = match source.kind() {
            SourceKind::Module => CompilerKind::Module,
            SourceKind::Script => CompilerKind::Script,
        };
        let compiler_stack = CompilerStack::new(kind.clone());
        let mut compiler = Self {
            syntax_tree,
            compiler_stack,
            source,
        };

        compiler.compile(kind).with_source(compiler.source)
    }

    fn compile(&mut self, kind: CompilerKind) -> Result<Bytecode, Error> {
        if let CompilerKind::Module = kind {
            self.context()?
                .scope_stack()
                .add_local(&EXPORT, State::initialized(false))?;
        }

        self.visit(self.syntax_tree.root())?;

        if let CompilerKind::Module = kind {
            let exports_slot = self
                .context()?
                .scope_stack()
                .local(&*EXPORT.get())
                .expect("#export should always be defined for modules.")
                .slot as u8;

            emit_bytecode! {
                self.assembler()?, Span::new(0..0) => [
                    POP;
                    LOAD_LOCAL exports_slot;
                ]
            };
        }

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context.finish(self.source.clone()))
    }

    pub(super) fn context(&mut self) -> Result<&mut CompilerContext, Error> {
        self.compiler_stack.top_mut()
    }

    pub(super) fn assembler(&mut self) -> Result<&mut Assembler, Error> {
        Ok(self.compiler_stack.top_mut()?.assembler())
    }
}
