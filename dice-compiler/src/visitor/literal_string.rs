use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::{bytecode::ConstantValue, error::Error};
use dice_syntax::LitString;

impl NodeVisitor<&LitString> for Compiler {
    fn visit(&mut self, LitString { value, span }: &LitString) -> Result<(), Error> {
        self.context()?
            .assembler()
            .push_const(ConstantValue::String(value.clone()), *span)?;

        Ok(())
    }
}
