use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::{
    error::{codes::VARIABLE_NOT_INITIALIZED, Error},
    error_tags,
    value::Symbol,
};
use dice_syntax::LitIdent;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent { identifier: name, span }: &LitIdent) -> Result<(), Error> {
        let name_symbol: Symbol = name.clone().into();

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(name_symbol.clone()) {
                if !scope_variable.is_initialized() {
                    return Err(Error::new(VARIABLE_NOT_INITIALIZED)
                        .with_span(*span)
                        .with_tags(error_tags! {
                            name => (&*scope_variable.name).to_owned()
                        }));
                }

                let slot = scope_variable.slot as u8;
                context.assembler().load_local(slot, *span);

                return Ok(());
            }
        }

        if let Some(upvalue) = self.compiler_stack.resolve_upvalue(name_symbol, 0) {
            let context = self.context()?;
            context.assembler().load_upvalue(upvalue as u8, *span);

            return Ok(());
        }

        self.assembler()?.load_global(&**name, *span)?;

        Ok(())
    }
}
