use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_stack::CompilerKind};
use dice_core::protocol::error::{IS_OK, RESULT};
use dice_error::compiler_error::CompilerError;
use dice_syntax::ErrorPropagate;

impl NodeVisitor<&ErrorPropagate> for Compiler {
    fn visit(&mut self, ErrorPropagate { expression, span }: &ErrorPropagate) -> Result<(), CompilerError> {
        if !matches!(self.context()?.kind(), CompilerKind::Function { .. }) {
            return Err(CompilerError::InvalidErrorPropagate);
        }

        self.visit(*expression)?;
        let error_propagate_jump;

        emit_bytecode! {
            self.assembler()?, *span => [
                DUP 0;
                LOAD_FIELD &IS_OK;
                JUMP_IF_TRUE -> error_propagate_jump;
                RET;
                {self.assembler()?.patch_jump(error_propagate_jump)};
                LOAD_FIELD &RESULT;
            ]
        }

        Ok(())
    }
}
