use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::State, upvalue::UpvalueDescriptor, visitor::FnKind};
use dice_core::value::{FnScript, Symbol, Value};
use dice_error::{compiler_error::CompilerError, span::Span};
use dice_syntax::{FnArg, FnDecl};
use std::collections::HashSet;

impl NodeVisitor<(&FnDecl, FnKind)> for Compiler {
    fn visit(&mut self, (fn_decl, fn_kind): (&FnDecl, FnKind)) -> Result<(), CompilerError> {
        Self::assert_unique_params(&fn_decl.args, fn_decl.span)?;

        let body = self.syntax_tree.child(fn_decl.body);
        let mut fn_context = self.compile_fn(body, &fn_decl.args, fn_decl.return_.clone(), fn_kind)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let compiled_fn = Value::FnScript(FnScript::new(&*fn_decl.name, bytecode, uuid::Uuid::new_v4()));

        if fn_kind == FnKind::Function {
            self.emit_fn(fn_decl, &upvalues, compiled_fn)?;
        } else {
            self.emit_method(fn_decl, &upvalues, compiled_fn)?;
        }

        Ok(())
    }
}

impl Compiler {
    pub(super) fn assert_unique_params(args: &[FnArg], span: Span) -> Result<(), CompilerError> {
        // NOTE: Assert that all arguments are uniquely named.
        let mut items = HashSet::with_capacity(args.len());
        let has_unique_args = args.iter().all(|arg| items.insert(&arg.name));

        if !has_unique_args {
            return Err(CompilerError::DuplicateArgumentNames(span));
        }

        Ok(())
    }

    fn emit_fn(
        &mut self,
        fn_decl: &FnDecl,
        upvalues: &[UpvalueDescriptor],
        compiled_fn: Value,
    ) -> Result<(), CompilerError> {
        let context = self.context()?;
        let fn_name: Symbol = (&*fn_decl.name).into();
        let local = context.scope_stack().local(fn_name).ok_or_else(|| {
            CompilerError::InternalCompilerError(String::from("Function not already declared in scope."))
        })?;

        // NOTE: Check if a function of the given name has already been initialized.
        match &mut local.state {
            State::Function { ref mut is_initialized } if *is_initialized => {
                return Err(CompilerError::ItemAlreadyDeclared(fn_decl.name.to_owned()))
            }
            State::Function { ref mut is_initialized } => *is_initialized = true,
            _ => unreachable!("Unexpected non-function local state while compiling a function."),
        }

        let slot = local.slot as u8;

        emit_bytecode! {
            self.assembler()?, fn_decl.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST compiled_fn;
                ] else [
                    CREATE_CLOSURE compiled_fn, upvalues;
                ]

                STORE_LOCAL slot;
            ]
        }

        Ok(())
    }

    fn emit_method(
        &mut self,
        fn_decl: &FnDecl,
        upvalues: &[UpvalueDescriptor],
        compiled_fn: Value,
    ) -> Result<(), CompilerError> {
        emit_bytecode! {
            self.assembler()?, fn_decl.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST compiled_fn;
                ] else [
                    CREATE_CLOSURE compiled_fn, upvalues;
                ]
            ]
        }

        Ok(())
    }
}
