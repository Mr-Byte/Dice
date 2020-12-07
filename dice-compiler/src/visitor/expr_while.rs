use super::{BlockKind, NodeVisitor};
use crate::{compiler::Compiler, compiler_error::CompilerError, scope_stack::ScopeKind};
use dice_core::span::Span;
use dice_syntax::{SyntaxNode, WhileLoop};

impl NodeVisitor<&WhileLoop> for Compiler {
    fn visit(&mut self, WhileLoop { condition, body, span }: &WhileLoop) -> Result<(), CompilerError> {
        if let SyntaxNode::Block(block) = self.syntax_tree.get(*body) {
            let block = block.clone();
            let loop_start = self.assembler()?.current_position();

            self.context()?
                .scope_stack()
                .push_scope(ScopeKind::Loop, Some(loop_start as usize));
            self.visit(*condition)?;

            let loop_end = self.assembler()?.jump_if_false(*span);

            self.visit((&block, BlockKind::Loop))?;
            self.assembler()?.jump_back(loop_start, *span);
            self.assembler()?.patch_jump(loop_end);

            let scope_close = self.context()?.scope_stack().pop_scope()?;

            for location in scope_close.exit_points.iter() {
                self.assembler()?.patch_jump(*location as u64);
            }

            self.assembler()?.push_unit(*span);
        } else {
            return Err(CompilerError::new(
                "ICE: While loop bodies should only ever contain blocks.",
                Span::empty(),
            ));
        }

        Ok(())
    }
}
