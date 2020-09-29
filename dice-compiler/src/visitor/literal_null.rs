use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::LitNull;

impl NodeVisitor<&LitNull> for Compiler {
    fn visit(&mut self, LitNull { span }: &LitNull) -> Result<(), CompilerError> {
        self.context()?.assembler().push_none(*span);

        Ok(())
    }
}
