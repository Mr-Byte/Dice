use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::Index;

impl NodeVisitor<&Index> for Compiler {
    fn visit(&mut self, node: &Index) -> Result<(), CompilerError> {
        self.visit(node.expression)?;

        // NOTE: Take the current call context and temporarily store it on the stack, replacing it with a new one, so that
        // any call chains associated with evaluating the index short-circuit only in the index. Once the index is
        // compiled, the original call context is restored, so further chained calls will shirt-circuit correctly.
        let original_call_context = std::mem::take(&mut self.context()?.scope_stack().top_mut()?.call_context);
        self.visit(node.index_expression)?;
        self.context()?.scope_stack().top_mut()?.call_context = original_call_context;
        self.context()?.assembler().load_index(node.span);

        Ok(())
    }
}
