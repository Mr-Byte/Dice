use crate::compiler::Compiler;
use crate::error::CompilerError;
use crate::scope_stack::State;
use crate::visitor::NodeVisitor;
use dice_syntax::ImportDecl;

impl NodeVisitor<&ImportDecl> for Compiler {
    fn visit(&mut self, node: &ImportDecl) -> Result<(), CompilerError> {
        let imports: Vec<(&str, u8)> = node
            .item_imports
            .iter()
            .map(|item| {
                let slot = self
                    .context()?
                    .scope_stack()
                    .add_local(item, State::initialized(false))?;

                Ok((item.as_str(), slot as u8))
            })
            .collect::<Result<Vec<_>, CompilerError>>()?;

        emit_bytecode! {
            self.context()?.assembler(), node.span => [
                LOAD_MODULE &node.relative_path;

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
                .add_local(module_import, State::initialized(false))?;

            self.context()?.assembler().store_local(module_slot as u8, node.span);
        }

        Ok(())
    }
}
