use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::FunctionCall;

impl NodeVisitor<&FunctionCall> for Compiler {
    fn visit(&mut self, node: &FunctionCall) -> Result<(), CompilerError> {
        self.visit(node.target)?;

        for arg in &node.args {
            self.visit(*arg)?;
        }

        self.context()?.assembler().call(node.args.len() as u8, node.span);

        Ok(())
    }
}
