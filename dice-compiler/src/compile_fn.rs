use crate::{
    compiler::Compiler,
    compiler_stack::{CompilerContext, CompilerKind},
    visitor::{FnKind, FunctionBlockKind, NodeVisitor},
};
use dice_core::error::{codes::NEW_METHOD_CANNOT_HAVE_RETURN_TYPE, Error};
use dice_syntax::{Block, FnArg, SyntaxNode, SyntaxTree, TypeAnnotation};

impl<'gc> Compiler {
    pub(crate) fn compile_fn(
        &mut self,
        syntax_tree: SyntaxTree,
        args: &[FnArg],
        return_type: Option<TypeAnnotation>,
        kind: FnKind,
    ) -> Result<CompilerContext, Error> {
        // NOTE: Constructors cannot have a return type annotation.
        if matches!(kind, FnKind::Constructor(_)) && return_type.is_some() {
            let err = Error::new(NEW_METHOD_CANNOT_HAVE_RETURN_TYPE).with_span(return_type.unwrap().span);
            return Err(err);
        }

        let compiler_kind = match kind {
            FnKind::Constructor(_) => CompilerKind::Constructor,
            FnKind::Method => CompilerKind::Method { return_type },
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
            FnKind::Constructor(class_kind) => FunctionBlockKind::Constructor(args, class_kind),
        };

        self.visit((&body, block_kind))?;

        let compiler_context = self.compiler_stack.pop()?;

        Ok(compiler_context)
    }
}
