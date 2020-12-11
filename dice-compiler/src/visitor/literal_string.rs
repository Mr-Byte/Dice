use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::{error::Error, value::Value};
use dice_syntax::LitString;

impl NodeVisitor<&LitString> for Compiler {
    fn visit(&mut self, LitString { value, span }: &LitString) -> Result<(), Error> {
        self.context()?
            .assembler()
            .push_const(Value::with_string(value), *span)?;

        Ok(())
    }
}
