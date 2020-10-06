use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::LitUnit;

impl NodeVisitor<&LitUnit> for Compiler {
    fn visit(&mut self, LitUnit { span }: &LitUnit) -> Result<(), CompilerError> {
        self.context()?.assembler().push_unit(*span);

        Ok(())
    }
}
