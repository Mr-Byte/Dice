use super::NodeVisitor;
use crate::compiler::Compiler;
use crate::error::CompilerError;
use dice_syntax::LitNone;

impl NodeVisitor<&LitNone> for Compiler {
    fn visit(&mut self, LitNone { span }: &LitNone) -> Result<(), CompilerError> {
        self.context()?.assembler().push_none(*span);

        Ok(())
    }
}
