use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::State};
use dice_core::value::{FnScript, Value};
use dice_error::compiler_error::CompilerError;
use dice_syntax::FnDecl;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FnKind {
    Function,
    Method,
}

impl NodeVisitor<(&FnDecl, FnKind)> for Compiler {
    fn visit(&mut self, (fn_decl, fn_kind): (&FnDecl, FnKind)) -> Result<(), CompilerError> {
        let body = self
            .syntax_tree
            .child(fn_decl.body)
            .expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &fn_decl.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(
            fn_decl.name.clone(),
            fn_decl.args.len(),
            bytecode,
            uuid::Uuid::new_v4(),
        ));
        let context = self.context()?;

        if fn_kind == FnKind::Function {
            let slot = {
                let fn_name = fn_decl.name.clone();
                let local = context.scope_stack().local(&fn_name).ok_or_else(|| {
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
                context.assembler(), fn_decl.span => [
                    if upvalues.is_empty() => [
                        PUSH_CONST value;
                    ] else [
                        CREATE_CLOSURE value, &upvalues;
                    ]

                    STORE_LOCAL slot;
                ]
            }
        } else {
            emit_bytecode! {
                context.assembler(), fn_decl.span => [
                    if upvalues.is_empty() => [
                        PUSH_CONST value;
                    ] else [
                        CREATE_CLOSURE value, &upvalues;
                    ]
                ]
            }
        }

        Ok(())
    }
}
