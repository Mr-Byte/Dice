use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::LitUnit;

impl NodeVisitor<&LitUnit> for Compiler {
    fn visit(&mut self, LitUnit { span }: &LitUnit) -> Result<(), Error> {
        self.assembler()?.push_unit(*span);

        Ok(())
    }
}
