use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::FunctionCall;

impl NodeVisitor<&FunctionCall> for Compiler {
    fn visit(&mut self, node: &FunctionCall) -> Result<(), CompilerError> {
        self.visit(node.target)?;

        // NOTE: Store the call depth at the time the function call was started, to be restored later.
        let starting_call_depth = *self.context()?.call_depth();
        for arg in &node.args {
            /* NOTE: Take the current call context and temporarily store it on the stack, replacing it with a new one, so that
             * any call chains associated with evaluating the argument short-circuit only in the argument. Once the argument is
             * compiled, the original call context is restored, so further chained calls will shirt-circuit correctly.
             */
            let original_call_context = std::mem::take(&mut self.context()?.scope_stack().top_mut()?.call_context);
            // NOTE: Increment call depth by 1 for each parameter.
            *self.context()?.call_depth() += 1;
            self.visit(*arg)?;
            self.context()?.scope_stack().top_mut()?.call_context = original_call_context;
        }

        *self.context()?.call_depth() = starting_call_depth;
        self.assembler()?.call(node.args.len() as u8, node.span);

        Ok(())
    }
}
