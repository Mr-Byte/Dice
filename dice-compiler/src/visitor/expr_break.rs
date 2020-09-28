use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::ScopeKind};
use dice_syntax::Break;

impl NodeVisitor<&Break> for Compiler {
    fn visit(&mut self, Break { span }: &Break) -> Result<(), CompilerError> {
        let context = self.context()?;

        if !context.scope_stack().in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidBreak);
        }

        let patch_location = context.assembler().jump(*span);
        context.scope_stack().add_loop_exit_point(patch_location as usize)?;

        Ok(())
    }
}
