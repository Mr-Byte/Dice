use crate::{compiler::Compiler, scope_stack::State};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{Block, ClassDecl, FnDecl, SyntaxNode, SyntaxNodeId};

impl Compiler {
    // NOTE: Scan through all the declared items in a block and add slots for any functions and classes ahead of time.
    // This allows functions and classes to refer to each other independent of declaration order.
    pub(super) fn scan_item_decls(&mut self, block: &Block) -> Result<(), CompilerError> {
        let expressions = block.expressions.iter().chain(block.trailing_expression.iter());
        for expression in expressions {
            self.scan_expr(*expression)?
        }

        Ok(())
    }

    fn scan_expr(&mut self, expression: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(expression).expect("Node should exist.");

        match node {
            SyntaxNode::FnDecl(fn_decl) => {
                let fn_decl = fn_decl.clone();
                self.scan_fn_decl(fn_decl)?;
            }
            SyntaxNode::ClassDecl(class_decl) => {
                let class_decl = class_decl.clone();
                self.scan_class_decl(class_decl)?;
            }
            SyntaxNode::ExportDecl(export) => {
                let export = export.clone();
                self.scan_expr(export.export)?
            }
            _ => {}
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

    fn scan_class_decl(&mut self, class_decl: ClassDecl) -> Result<(), CompilerError> {
        let name = class_decl.name;
        self.context()?
            .scope_stack()
            .add_local(name, State::Class { is_initialized: false })?;

        Ok(())
    }
}
