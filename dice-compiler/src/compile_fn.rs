use crate::{
    compiler::Compiler,
    compiler_stack::{CompilerContext, CompilerKind},
    visitor::{BlockKind, FnKind, NodeVisitor},
};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{Block, SyntaxNode, SyntaxTree};

impl Compiler {
    pub(crate) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        args: &[impl AsRef<str>],
        kind: FnKind,
    ) -> Result<CompilerContext, CompilerError> {
        let compiler_kind = match kind {
            FnKind::Constructor => CompilerKind::Constructor,
            _ => CompilerKind::Function,
        };

        self.compiler_stack.push(compiler_kind);

        let root = syntax_tree.get(syntax_tree.root());
        let body = if let SyntaxNode::Block(body) = root {
            body.clone()
        } else {
            Block {
                expressions: Vec::new(),
                trailing_expression: Some(syntax_tree.root()),
                span: syntax_tree.get(syntax_tree.root()).span(),
            }
        };
        let block_kind = match kind {
            FnKind::Function | FnKind::StaticMethod => BlockKind::Function(args),
            FnKind::Method => BlockKind::Method(args),
            FnKind::Constructor => BlockKind::Constructor(args),
        };

        self.visit((&body, block_kind))?;

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context)
    }
}
