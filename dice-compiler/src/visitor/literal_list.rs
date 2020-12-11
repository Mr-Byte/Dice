use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::error::Error;
use dice_syntax::LitList;

impl NodeVisitor<&LitList> for Compiler {
    fn visit(&mut self, LitList { items: value, span }: &LitList) -> Result<(), Error> {
        for item in value {
            self.visit(*item)?;
        }

        self.assembler()?.create_list(value.len() as u8, *span);

        Ok(())
    }
}
