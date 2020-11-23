use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_stack::CompilerKind};
use dice_core::protocol::{
    class::{SELF, SUPER},
    ProtocolSymbol,
};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{LitIdent, SuperAccess};

impl NodeVisitor<&SuperAccess> for Compiler {
    fn visit(
        &mut self,
        SuperAccess {
            field,
            super_class,
            span,
        }: &SuperAccess,
    ) -> Result<(), CompilerError> {
        if !matches!(self.context()?.kind(), CompilerKind::Method { .. } | CompilerKind::Constructor) {
            return Err(CompilerError::InvalidSuperAccess(*span));
        }

        match super_class {
            Some(super_class) => {
                emit_bytecode! {
                    self.assembler()?, *span => [
                         {self.visit(&LitIdent::synthesize(super_class, *span))?};
                         {self.visit(&LitIdent::synthesize(SELF.get(), *span))?};
                         // TODO: Assert self is a subtype of super_class
                    ]
                }
            }
            None => {
                self.visit(&LitIdent::synthesize(SUPER.get(), *span))?;
                self.visit(&LitIdent::synthesize(SELF.get(), *span))?;
            }
        }

        self.assembler()?.load_method(&**field, *span)?;

        Ok(())
    }
}
