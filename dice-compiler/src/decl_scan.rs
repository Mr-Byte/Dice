use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_syntax::{Block, FnDecl, SyntaxNode, SyntaxNodeId};

impl Compiler {
    // NOTE: through all the declared items in a block and add slots for any functions and classes ahead of time.
    // This allows functions and classes to refer to each other independent of declaration order.
    pub(super) fn scan_item_decls(&mut self, block: &Block) -> Result<(), CompilerError> {
        for expression in &block.expressions {
            self.scan_expr(*expression)?
        }

        if let Some(trailing_expression) = block.trailing_expression {
            self.scan_expr(trailing_expression)?
        }

        Ok(())
    }

    fn scan_expr(&mut self, expression: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(expression).expect("Node should exist.");

        if let SyntaxNode::FnDecl(fn_decl) = node {
            let fn_decl = fn_decl.clone();
            self.scan_fn_decl(fn_decl)?;
        } else if let SyntaxNode::ExportDecl(export) = node {
            let export = export.clone();
            self.scan_expr(export.export)?
        }

        Ok(())
    }

    fn scan_fn_decl(&mut self, fn_decl: FnDecl) -> Result<(), CompilerError> {
        let name = fn_decl.name;
        self.context()?
            .scope_stack()
            .add_local(name, State::Function { is_initialized: false })?;

        Ok(())
    }
}
