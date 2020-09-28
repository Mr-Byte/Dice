use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::{Unary, UnaryOperator};

impl NodeVisitor<&Unary> for Compiler {
    fn visit(
        &mut self,
        Unary {
            operator: op,
            expression: expr,
            span,
        }: &Unary,
    ) -> Result<(), CompilerError> {
        self.visit(*expr)?;

        match op {
            UnaryOperator::Negate => self.context()?.assembler().neg(*span),
            UnaryOperator::Not => self.context()?.assembler().not(*span),
            UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }
}
