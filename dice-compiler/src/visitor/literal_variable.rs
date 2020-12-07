use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_error::CompilerError};
use dice_core::value::Symbol;
use dice_syntax::LitIdent;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent { name, span }: &LitIdent) -> Result<(), CompilerError> {
        let name_symbol: Symbol = name.clone().into();

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(name_symbol.clone()) {
                if !scope_variable.is_initialized() {
                    return Err(CompilerError::new(
                        format!(
                            "The variable {} is not initialized.",
                            (&*scope_variable.name).to_owned()
                        ),
                        *span,
                    ));
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
