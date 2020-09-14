use crate::{compiler::Compiler, syntax::LitString, CompilerError, Value};

use super::NodeVisitor;

impl NodeVisitor<&LitString> for Compiler {
    fn visit(&mut self, LitString(value, span): &LitString) -> Result<(), CompilerError> {
        self.current_assembler()
            .push_const(Value::String(value.clone()), span.clone());

        Ok(())
    }
}
