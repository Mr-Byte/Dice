use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::FunctionCall;

impl NodeVisitor<&FunctionCall> for Compiler {
    fn visit(&mut self, node: &FunctionCall) -> Result<(), CompilerError> {
        self.visit(node.target)?;

        for arg in &node.args {
            // NOTE: Take the current call context and temporarily store it on the stack, replacing it with a new one, so that
            // any call chains associated with evaluating the argument short-circuit only in the argument. Once the argument is
            // compiled, the original call context is restored, so further chained calls will shirt-circuit correctly.
            let original_call_context = std::mem::take(&mut self.context()?.scope_stack().top_mut()?.call_context);
            self.visit(*arg)?;
            self.context()?.scope_stack().top_mut()?.call_context = original_call_context;
        }

        self.context()?.assembler().call(node.args.len() as u8, node.span);

        Ok(())
    }
}
