use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_stack::CompilerKind};
use dice_core::{
    error::{codes::INVALID_SUPER_CALL, Error},
    protocol::{
        class::{SELF, SUPER},
        ProtocolSymbol,
    },
};
use dice_syntax::{LitIdent, SuperAccess};

impl NodeVisitor<&SuperAccess> for Compiler {
    fn visit(
        &mut self,
        SuperAccess {
            field,
            super_class,
            span,
        }: &SuperAccess,
    ) -> Result<(), Error> {
        if !matches!(
            self.context()?.kind(),
            CompilerKind::Method { .. } | CompilerKind::Constructor
        ) {
            return Err(Error::new(INVALID_SUPER_CALL).with_span(*span));
        }

        match super_class {
            Some(super_class) => self.visit(&LitIdent::synthesize(super_class, *span))?,
            None => self.visit(&LitIdent::synthesize(SUPER.get(), *span))?,
        }

        self.visit(&LitIdent::synthesize(SELF.get(), *span))?;
        self.assembler()?.load_method(&**field, *span)?;

        Ok(())
    }
}
