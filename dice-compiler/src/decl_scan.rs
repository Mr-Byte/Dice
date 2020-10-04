use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_syntax::{Block, FnDecl, SyntaxNode};

impl Compiler {
    /// Scan through all the declared items in a block and add slots for any functions and classes ahead of time.
    pub(super) fn scan_item_decls(&mut self, block: &Block) -> Result<(), CompilerError> {
        // TODO: Make the scan dispatcher a function.
        for expression in &block.expressions {
            match self.syntax_tree.get(*expression) {
                Some(SyntaxNode::FnDecl(fn_decl)) => {
                    let fn_decl = fn_decl.clone();
                    self.fn_decl(fn_decl)?
                }
                Some(SyntaxNode::ExportDecl(export)) => match self.syntax_tree.get(export.export) {
                    Some(SyntaxNode::FnDecl(fn_decl)) => {
                        let fn_decl = fn_decl.clone();
                        self.fn_decl(fn_decl)?
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(trailing_expression) = block.trailing_expression {
            match self.syntax_tree.get(trailing_expression) {
                Some(SyntaxNode::FnDecl(fn_decl)) => {
                    let fn_decl = fn_decl.clone();
                    self.fn_decl(fn_decl)?
                }
                Some(SyntaxNode::ExportDecl(export)) => match self.syntax_tree.get(export.export) {
                    Some(SyntaxNode::FnDecl(fn_decl)) => {
                        let fn_decl = fn_decl.clone();
                        self.fn_decl(fn_decl)?
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn fn_decl(&mut self, fn_decl: FnDecl) -> Result<(), CompilerError> {
        let name = fn_decl.name;
        self.context()?
            .scope_stack()
            .add_local(name, State::Function { is_initialized: false })?;

        Ok(())
    }
}
