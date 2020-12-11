use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::LitBool;

impl NodeVisitor<&LitBool> for Compiler {
    fn visit(&mut self, LitBool { value, span }: &LitBool) -> Result<(), Error> {
        self.compiler_stack.top_mut()?.assembler().push_bool(*value, *span);

        Ok(())
    }
}
