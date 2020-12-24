use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::State};
use dice_core::error::Error;
use dice_syntax::{VarDecl, VarDeclKind};

impl NodeVisitor<&VarDecl> for Compiler {
    fn visit(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
        self.visit(var_decl.expr)?;

        match &var_decl.kind {
            VarDeclKind::Singular(name) => self.singular_var(var_decl, name),
            VarDeclKind::Destructured(variables) => self.destructured_var(var_decl, variables),
        }
    }
}

impl Compiler {
    fn singular_var(&mut self, var_decl: &VarDecl, name: &str) -> Result<(), Error> {
        let slot = self
            .context()?
            .scope_stack()
            .add_local(name, State::initialized(var_decl.is_mutable))? as u8;

        emit_bytecode! {
            self.assembler()?, var_decl.span => [
                STORE_LOCAL slot;
            ]
        }

        if let Some(type_) = &var_decl.type_ {
            emit_bytecode! {
                self.assembler()?, var_decl.span => [
                    {self.visit(&type_.name)?};
                    if type_.is_nullable => [
                        ASSERT_TYPE_OR_NULL_FOR_LOCAL slot;
                    ] else [
                        ASSERT_TYPE_FOR_LOCAL slot;
                    ]
                ]
            }
        }

        Ok(())
    }
}

impl Compiler {
    fn destructured_var(&mut self, var_decl: &VarDecl, variables: &[String]) -> Result<(), Error> {
        let imports: Vec<(&str, u8)> = variables
            .iter()
            .map(|item| {
                let slot = self
                    .context()?
                    .scope_stack()
                    .add_local(item.clone(), State::initialized(false))?;

                Ok((item.as_str(), slot as u8))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        emit_bytecode! {
            self.assembler()?, var_decl.span => [
                for (field, slot) in imports => [
                    DUP 0;
                    LOAD_FIELD field;
                    STORE_LOCAL slot;
                    POP;
                ]
            ]
        }

        Ok(())
    }
}
