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
        let safe_access_jump;

        emit_bytecode! {
            self.context()?.assembler(), *span => [
                DUP 0;
                PUSH_NULL;
                NEQ;
                JUMP_IF_FALSE -> safe_access_jump;
                LOAD_FIELD field;
            ]
        }

        self.context()?
            .scope_stack()
            .top_mut()?
            .call_context
            .exit_points
            .push(safe_access_jump as usize);

        Ok(())
    }
}
