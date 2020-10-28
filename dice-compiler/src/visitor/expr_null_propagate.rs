use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::NullPropagate;

impl NodeVisitor<&NullPropagate> for Compiler {
    fn visit(&mut self, NullPropagate { expression, span }: &NullPropagate) -> Result<(), CompilerError> {
        self.visit(*expression)?;
        let null_propagate_jump;

        emit_bytecode! {
            self.assembler()?, *span => [
                DUP 0;
                PUSH_NULL;
                NEQ;
                JUMP_IF_FALSE -> null_propagate_jump;
            ]
        }

        self.context()?
            .scope_stack()
            .top_mut()?
            .call_context
            .exit_points
            .push(null_propagate_jump as usize);

        Ok(())
    }
}
