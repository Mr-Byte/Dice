use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_error::CompilerError};
use dice_syntax::FnCall;

impl NodeVisitor<&FnCall> for Compiler {
    fn visit(&mut self, node: &FnCall) -> Result<(), CompilerError> {
        self.visit(node.target)?;

        // NOTE: Store the temporary at the time the function call was started, to be restored later.
        let original_temporary_count = *self.context()?.temporary_count();
        for arg in &node.args {
            /* NOTE: Take the current call context and temporarily store it on the stack, replacing it with a new one, so that
             * any call chains associated with evaluating the argument short-circuit only in the argument. Once the argument is
             * compiled, the original call context is restored, so further chained calls will shirt-circuit correctly.
             */
            let original_call_context = std::mem::take(&mut self.context()?.scope_stack().top_mut()?.call_context);
            // NOTE: Increment temporary by 1 for each parameter.
            *self.context()?.temporary_count() += 1;
            self.visit(*arg)?;
            self.context()?.scope_stack().top_mut()?.call_context = original_call_context;
        }

        *self.context()?.temporary_count() = original_temporary_count;
        self.assembler()?.call(node.args.len() as u8, node.span);

        Ok(())
    }
}
