use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_stack::CompilerKind};
use dice_error::compiler_error::CompilerError;
use dice_syntax::Return;

impl NodeVisitor<&Return> for Compiler {
    fn visit(&mut self, expr_return: &Return) -> Result<(), CompilerError> {
        let context = self.context()?;

        if !matches!(context.kind(), CompilerKind::Function { .. }) {
            return Err(CompilerError::InvalidReturn);
        }

        // TODO: Allow expressionless return inside of constructors.
        match expr_return.result {
            Some(expr) => self.visit(expr)?,
            None => context.assembler().push_unit(expr_return.span),
        }

        // NOTE: Cleanup any temporaries created while calling functions then return.
        let temporary_count = *self.context()?.temporary_count();
        emit_bytecode! {
            self.assembler()?, expr_return.span => [
                for _ in 0..temporary_count => [
                    SWAP;
                    POP;
                ]
                RET;
            ]
        }

        Ok(())
    }
}
