use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_syntax::VarDecl;

impl NodeVisitor<&VarDecl> for Compiler {
    fn visit(&mut self, var_decl: &VarDecl) -> Result<(), CompilerError> {
        self.visit(var_decl.expr)?;

        let name = var_decl.name.clone();
        let slot = self.context()?.scope_stack().add_local(
            name,
            State::Local {
                is_mutable: var_decl.is_mutable,
                is_initialized: true, // TODO: Once initialization can be split from declaration mark this accordingly.
            },
        )? as u8;

        self.context()?.assembler().store_local(slot, var_decl.span);

        Ok(())
    }
}
