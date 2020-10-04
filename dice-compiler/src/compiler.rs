use crate::{
    compiler_stack::{CompilerContext, CompilerKind, CompilerStack},
    error::CompilerError,
    scope_stack::State,
    visitor::{BlockKind, NodeVisitor},
};
use dice_core::{bytecode::Bytecode, constants::EXPORT};
use dice_syntax::{Block, Parser, Span, SyntaxNode, SyntaxTree};
use std::path::Path;

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum CompilationKind {
    Script,
    Module,
}

pub struct Compiler {
    pub(crate) syntax_tree: SyntaxTree,
    pub(crate) compiler_stack: CompilerStack,
}

impl Compiler {
    fn new(syntax_tree: SyntaxTree, kind: CompilationKind) -> Self {
        let compiler_kind = match kind {
            CompilationKind::Script => CompilerKind::Script,
            CompilationKind::Module => CompilerKind::Module,
        };
        let compiler_stack = CompilerStack::new(compiler_kind);

        Self {
            syntax_tree,
            compiler_stack,
        }
    }

    pub fn compile_module(path: &Path) -> Result<Bytecode, CompilerError> {
        let source = std::fs::read_to_string(path)?;

        Self::compile_str(&source, CompilationKind::Module)
    }

    pub fn compile_str(input: &str, kind: CompilationKind) -> Result<Bytecode, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self::new(syntax_tree, kind);

        if kind == CompilationKind::Module {
            compiler
                .context()?
                .scope_stack()
                .add_local(EXPORT, State::initialized(false))?;
        }

        compiler.visit(compiler.syntax_tree.root())?;

        if kind == CompilationKind::Module {
            let exports_slot = compiler
                .context()?
                .scope_stack()
                .local(EXPORT)
                .expect("#export should always be defined for modules.")
                .slot as u8;

            emit_bytecode! {
                compiler.context()?.assembler(), Span::new(0..0) => [
                    POP;
                    LOAD_LOCAL exports_slot;
                ]
            };
        }

        let compiler_context = compiler.compiler_stack.pop()?;

        Ok(compiler_context.finish())
    }

    pub(crate) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        args: &[impl AsRef<str>],
    ) -> Result<CompilerContext, CompilerError> {
        self.compiler_stack.push(CompilerKind::Function);

        let root = syntax_tree.get(syntax_tree.root()).expect("Node should not be empty");

        let body = if let SyntaxNode::Block(body) = root {
            body.clone()
        } else {
            Block {
                expressions: Vec::new(),
                trailing_expression: Some(syntax_tree.root()),
                span: syntax_tree.get(syntax_tree.root()).expect("Node should exist.").span(),
            }
        };

        self.visit((&body, BlockKind::Function(args)))?;

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context)
    }

    pub(crate) fn context(&mut self) -> Result<&mut CompilerContext, CompilerError> {
        self.compiler_stack.top_mut()
    }
}
