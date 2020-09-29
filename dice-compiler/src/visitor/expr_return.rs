use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_stack::CompilerKind, error::CompilerError};
use dice_syntax::Return;

impl NodeVisitor<&Return> for Compiler {
    fn visit(&mut self, expr_return: &Return) -> Result<(), CompilerError> {
        let context = self.context()?;

        if !matches!(context.kind(), CompilerKind::Function { .. }) {
            return Err(CompilerError::InvalidReturn);
        }

        match expr_return.result {
            Some(expr) => self.visit(expr)?,
            None => context.assembler().push_unit(expr_return.span),
        }

        self.context()?.assembler().ret(expr_return.span);

        Ok(())
    }
}
