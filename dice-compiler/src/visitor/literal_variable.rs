use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_error::compiler_error::CompilerError;
use dice_syntax::LitIdent;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent { name, span }: &LitIdent) -> Result<(), CompilerError> {
        let name_symbol = name.clone();

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(&name_symbol) {
                if !scope_variable.is_initialized() {
                    return Err(CompilerError::UninitializedVariable(scope_variable.name.clone()));
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

        self.context()?.assembler().load_global(&**name, *span)?;

        Ok(())
    }
}
