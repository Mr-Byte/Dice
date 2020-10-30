use crate::{
    compiler::Compiler,
    compiler_stack::{CompilerContext, CompilerKind},
    visitor::{FnKind, FunctionBlockKind, NodeVisitor},
};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{Block, FnArg, SyntaxNode, SyntaxTree, TypeAnnotation};

impl Compiler {
    pub(crate) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        args: &[FnArg],
        return_type: Option<TypeAnnotation>,
        kind: FnKind,
    ) -> Result<CompilerContext, CompilerError> {
        let compiler_kind = match kind {
            FnKind::Constructor => CompilerKind::Constructor,
            _ => CompilerKind::Function { return_type },
        };

        self.compiler_stack.push(compiler_kind);

        let root = syntax_tree.get(syntax_tree.root());
        let body = match root {
            SyntaxNode::Block(body) => body.clone(),
            _ => Block {
                expressions: Vec::new(),
                trailing_expression: Some(syntax_tree.root()),
                span: syntax_tree.get(syntax_tree.root()).span(),
            },
        };
        let block_kind = match kind {
            FnKind::Function | FnKind::StaticMethod => FunctionBlockKind::Function(args),
            FnKind::Method => FunctionBlockKind::Method(args),
            FnKind::Constructor => FunctionBlockKind::Constructor(args),
        };

        self.visit((&body, block_kind))?;

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context)
    }
}
