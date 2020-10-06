use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::LitObject;

impl NodeVisitor<&LitObject> for Compiler {
    fn visit(&mut self, LitObject { items, span }: &LitObject) -> Result<(), CompilerError> {
        // TODO: Generate actual type ids.
        self.context()?.assembler().create_object(0, *span);

        for (field, value) in items {
            self.context()?.assembler().dup(0, *span);
            self.visit(*value)?;
            self.context()?.assembler().store_field(field, *span)?;
            self.context()?.assembler().pop(*span);
        }

        Ok(())
    }
}
