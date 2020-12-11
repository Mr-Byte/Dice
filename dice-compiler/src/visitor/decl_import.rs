use crate::{compiler::Compiler, scope_stack::State, visitor::NodeVisitor};
use dice_core::error::Error;
use dice_syntax::ImportDecl;

impl NodeVisitor<&ImportDecl> for Compiler {
    fn visit(&mut self, node: &ImportDecl) -> Result<(), Error> {
        let imports: Vec<(&str, u8)> = node
            .item_imports
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
            self.assembler()?, node.span => [
                LOAD_MODULE &*node.relative_path;

                for (field, slot) in imports => [
                    DUP 0;
                    LOAD_FIELD field;
                    STORE_LOCAL slot;
                    POP;
                ]
            ]
        }

        if let Some(module_import) = &node.module_import {
            let module_slot = self
                .context()?
                .scope_stack()
                .add_local(module_import.clone(), State::initialized(false))?;

            self.assembler()?.store_local(module_slot as u8, node.span);
        }

        Ok(())
    }
}
