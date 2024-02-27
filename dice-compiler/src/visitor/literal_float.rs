use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::{bytecode::ConstantValue, error::Error};
use dice_syntax::LitFloat;

impl NodeVisitor<&LitFloat> for Compiler {
    fn visit(&mut self, LitFloat { value, span }: &LitFloat) -> Result<(), Error> {
        let context = self.context()?;

        if *value == 0.0 {
            context.assembler().push_f0(*span);
        } else if *value == 1.0 {
            context.assembler().push_f1(*span);
        } else {
            context.assembler().push_const(ConstantValue::Float(*value), *span)?;
        }

        Ok(())
    }
}
