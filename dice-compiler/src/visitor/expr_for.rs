use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::ForLoop;

impl NodeVisitor<&ForLoop> for Compiler {
    fn visit(&mut self, _for_loop: &ForLoop) -> Result<(), CompilerError> {
        if let Some(range_loop) = self.lower_range_loop(_for_loop) {
            return self.visit(&range_loop);
        }

        Ok(())
    }
}
