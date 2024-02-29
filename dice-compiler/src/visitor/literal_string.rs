use dice_bytecode::ConstantValue;
use dice_core::error::Error;
use dice_syntax::LitString;

use crate::compiler::Compiler;

use super::NodeVisitor;

impl NodeVisitor<&LitString> for Compiler {
    fn visit(&mut self, LitString { value, span }: &LitString) -> Result<(), Error> {
        self.context()?
            .assembler()
            .push_const(ConstantValue::String(value.clone()), *span)?;

        Ok(())
    }
}
