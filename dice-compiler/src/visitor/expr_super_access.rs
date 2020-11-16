use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::{LitIdent, SuperAccess};

impl NodeVisitor<&SuperAccess> for Compiler {
    fn visit(&mut self, SuperAccess { field, span }: &SuperAccess) -> Result<(), CompilerError> {
        self.visit(&LitIdent {
            name: "super".to_owned(),
            span: *span,
        })?;
        self.visit(&LitIdent {
            name: "self".to_owned(),
            span: *span,
        })?;
        self.assembler()?.load_method(&**field, *span)?;

        Ok(())
    }
}
