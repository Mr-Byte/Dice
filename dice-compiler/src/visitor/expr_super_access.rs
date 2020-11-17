use super::NodeVisitor;
use crate::compiler::Compiler;
use crate::compiler_stack::CompilerKind;
use dice_core::protocol::{
    class::{SELF, SUPER},
    ProtocolSymbol,
};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{LitIdent, SuperAccess};

impl NodeVisitor<&SuperAccess> for Compiler {
    fn visit(&mut self, SuperAccess { field, span }: &SuperAccess) -> Result<(), CompilerError> {
        if !matches!(self.context()?.kind(), CompilerKind::Method { .. } | CompilerKind::Constructor) {
            return Err(CompilerError::InvalidSuperAccess(*span));
        }

        self.visit(&LitIdent {
            name: SUPER.get().to_string(),
            span: *span,
        })?;
        self.visit(&LitIdent {
            name: SELF.get().to_string(),
            span: *span,
        })?;
        self.assembler()?.load_method(&**field, *span)?;

        Ok(())
    }
}
