use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_error::CompilerError, compiler_stack::CompilerKind};
use dice_core::protocol::class::SELF;
use dice_syntax::Return;

impl NodeVisitor<&Return> for Compiler {
    fn visit(&mut self, expr_return: &Return) -> Result<(), CompilerError> {
        let context = self.context()?;

        match context.kind() {
            CompilerKind::Function { .. } | CompilerKind::Method { .. } => match expr_return.result {
                Some(expr) => self.visit(expr)?,
                None => context.assembler().push_unit(expr_return.span),
            },
            CompilerKind::Constructor if expr_return.result.is_none() => {
                let self_slot = context
                    .scope_stack()
                    .local(&SELF)
                    .expect("The self parameter should always be declared in constructors.")
                    .slot;
                self.assembler()?.load_local(self_slot as u8, expr_return.span);
            }
            CompilerKind::Constructor if expr_return.result.is_some() => {
                return Err(CompilerError::new(
                    "The return keyword cannot be used with an expression in constructors.",
                    expr_return.span,
                ))
            }
            _ => {
                return Err(CompilerError::new(
                    "The return keyword can only be used inside functions.",
                    expr_return.span,
                ))
            }
        }

        // NOTE: Cleanup any temporaries created while calling functions then return.
        let temporary_count = *self.context()?.temporary_count();

        emit_bytecode! {
            self.assembler()?, expr_return.span => [
                for _ in 0..temporary_count => [
                    SWAP;
                    POP;
                ]
                {self.visit_return(expr_return.span)?};
            ]
        }

        Ok(())
    }
}
