use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::SuperCall;

impl NodeVisitor<&SuperCall> for Compiler {
    fn visit(&mut self, _node: &SuperCall) -> Result<(), CompilerError> {
        Ok(())
    }
}
