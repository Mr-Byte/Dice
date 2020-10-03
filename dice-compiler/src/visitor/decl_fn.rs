use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_core::value::{FnScript, Value};
use dice_syntax::FnDecl;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &node.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(
            node.name.clone(),
            node.args.len(),
            bytecode,
            uuid::Uuid::new_v4(),
        ));
        let context = self.context()?;
        let slot = {
            let fn_name = node.name.clone();
            let local = context.scope_stack().local(fn_name.clone()).ok_or_else(|| {
                CompilerError::InternalCompilerError(String::from("Function not already declared in scope."))
            })?;

            // NOTE: Check if a function of the given name has already been initialized.
            if let State::Function { ref mut is_initialized } = &mut local.state {
                if *is_initialized {
                    return Err(CompilerError::ItemAlreadyDeclared(fn_name));
                }

                *is_initialized = true;
            }

            local.slot as u8
        };

        emit_bytecode! {
            context.assembler(), node.span => [
                when upvalues.is_empty() => [
                    PUSH_CONST value;
                ] else [
                    CLOSURE value, &upvalues;
                ]

                STORE_LOCAL slot;
            ]
        }

        Ok(())
    }
}
