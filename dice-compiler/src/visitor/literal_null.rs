use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::LitNull;

impl NodeVisitor<&LitNull> for Compiler {
    fn visit(&mut self, LitNull { span }: &LitNull) -> Result<(), Error> {
        self.assembler()?.push_null(*span);

        Ok(())
    }
}
