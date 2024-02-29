use dice_bytecode::ConstantValue;
use dice_core::error::Error;
use dice_syntax::LitInt;

use crate::compiler::Compiler;

use super::NodeVisitor;

impl NodeVisitor<&LitInt> for Compiler {
    fn visit(&mut self, LitInt { value, span }: &LitInt) -> Result<(), Error> {
        let context = self.context()?;

        match value {
            0 => context.assembler().push_i0(*span),
            1 => context.assembler().push_i1(*span),
            _ => context.assembler().push_const(ConstantValue::Int(*value), *span)?,
        }

        Ok(())
    }
}
