use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::FieldAccess;

impl NodeVisitor<&FieldAccess> for Compiler {
    fn visit(
        &mut self,
        FieldAccess {
            expression,
            field,
            span,
        }: &FieldAccess,
    ) -> Result<(), CompilerError> {
        self.visit(*expression)?;
        self.context()?.assembler().load_field(field, *span)?;

        Ok(())
    }
}
