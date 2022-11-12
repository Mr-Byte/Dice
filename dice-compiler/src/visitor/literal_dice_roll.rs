use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::LitDiceRoll;

impl NodeVisitor<&LitDiceRoll> for Compiler {
    fn visit(&mut self, LitDiceRoll { span }: &LitDiceRoll) -> Result<(), Error> {
        let context = self.context()?;

        // TODO: Actually evulate dice rolls LULs
        context.assembler().push_f0(*span);

        Ok(())
    }
}
