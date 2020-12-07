use super::NodeVisitor;
use crate::compiler::Compiler;
use crate::compiler_error::CompilerError;
use dice_syntax::LitBool;

impl NodeVisitor<&LitBool> for Compiler {
    fn visit(&mut self, LitBool { value, span }: &LitBool) -> Result<(), CompilerError> {
        self.compiler_stack.top_mut()?.assembler().push_bool(*value, *span);

        Ok(())
    }
}
