use super::NodeVisitor;
use crate::compiler::Compiler;
use crate::compiler_error::CompilerError;
use dice_syntax::Is;

impl NodeVisitor<&Is> for Compiler {
    fn visit(&mut self, Is { value, type_, span }: &Is) -> Result<(), CompilerError> {
        if type_.is_nullable {
            // TODO: Replace this with an IS_TYPE_OR_NULL instruction.
            let type_check_jump;
            emit_bytecode! {
                self.assembler()?, *span => [
                    {self.visit(*value)?};
                    PUSH_NULL;
                    EQ;
                    DUP 0;
                    JUMP_IF_TRUE -> type_check_jump;
                    POP;
                    {self.visit(*value)?};
                    {self.visit(&type_.name)?};
                    IS;
                    PATCH_JUMP <- type_check_jump;
                ]
            }
        } else {
            self.visit(*value)?;
            self.visit(&type_.name)?;
            self.assembler()?.is(*span);
        }

        Ok(())
    }
}
