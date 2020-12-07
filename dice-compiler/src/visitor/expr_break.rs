use super::NodeVisitor;
use crate::compiler_error::CompilerError;
use crate::{compiler::Compiler, scope_stack::ScopeKind};
use dice_syntax::Break;

impl NodeVisitor<&Break> for Compiler {
    fn visit(&mut self, Break { span }: &Break) -> Result<(), CompilerError> {
        let context = self.context()?;

        if !context.scope_stack().in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::new(
                "The break keyword can only be used inside loops.",
                *span,
            ));
        }

        let patch_location = context.assembler().jump(*span);
        context.scope_stack().add_loop_exit_point(patch_location as usize)?;

        Ok(())
    }
}
