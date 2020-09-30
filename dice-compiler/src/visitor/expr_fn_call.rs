use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::FunctionCall;

impl NodeVisitor<&FunctionCall> for Compiler {
    fn visit(&mut self, node: &FunctionCall) -> Result<(), CompilerError> {
        self.visit(node.target)?;

        for arg in &node.args {
            let original_call_context = std::mem::take(&mut self.context()?.scope_stack().top_mut()?.call_context);
            self.visit(*arg)?;
            self.context()?.scope_stack().top_mut()?.call_context = original_call_context;
        }

        self.context()?.assembler().call(node.args.len() as u8, node.span);

        Ok(())
    }
}
