use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::SafeAccess;

impl NodeVisitor<&SafeAccess> for Compiler {
    fn visit(
        &mut self,
        SafeAccess {
            expression,
            field,
            span,
        }: &SafeAccess,
    ) -> Result<(), CompilerError> {
        self.visit(*expression)?;
        self.context()?.assembler().dup(*span);
        self.context()?.assembler().push_none(*span);
        self.context()?.assembler().neq(*span);

        let safe_access_jump = self.context()?.assembler().jump_if_false(*span);
        self.context()?
            .scope_stack()
            .top_mut()?
            .call_context
            .exit_points
            .push(safe_access_jump as usize);
        self.context()?.assembler().load_field(field, *span)?;

        Ok(())
    }
}
