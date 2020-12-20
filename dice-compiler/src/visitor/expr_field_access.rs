use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::FieldAccess;

impl NodeVisitor<&FieldAccess> for Compiler {
    fn visit(&mut self, FieldAccess { expression, field, span }: &FieldAccess) -> Result<(), Error> {
        self.visit(*expression)?;
        self.assembler()?.load_field(&**field, *span)?;

        Ok(())
    }
}
