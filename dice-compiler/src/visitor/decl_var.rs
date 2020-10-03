use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_syntax::VarDecl;

impl NodeVisitor<&VarDecl> for Compiler {
    fn visit(&mut self, var_decl: &VarDecl) -> Result<(), CompilerError> {
        self.visit(var_decl.expr)?;

        let name = var_decl.name.clone();
        let slot = self
            .context()?
            .scope_stack()
            .add_local(name, State::initialized(var_decl.is_mutable))? as u8;

        emit_bytecode! {
            self.context()?.assembler(), var_decl.span => [
                STORE_LOCAL slot;
            ]
        }

        Ok(())
    }
}
