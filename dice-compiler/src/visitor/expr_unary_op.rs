use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_core::operator::DIE_ROLL;
use dice_syntax::{Span, SyntaxNodeId, Unary, UnaryOperator};

impl NodeVisitor<&Unary> for Compiler {
    fn visit(
        &mut self,
        Unary {
            operator,
            expression,
            span,
        }: &Unary,
    ) -> Result<(), CompilerError> {
        match operator {
            UnaryOperator::Negate => self.negate(*expression, *span),
            UnaryOperator::Not => self.not(*expression, *span),
            UnaryOperator::DiceRoll => self.die_roll(*expression, *span),
        }
    }
}

impl Compiler {
    fn negate(&mut self, expression: SyntaxNodeId, span: Span) -> Result<(), CompilerError> {
        self.visit(expression)?;
        self.context()?.assembler().neg(span);

        Ok(())
    }

    fn not(&mut self, expression: SyntaxNodeId, span: Span) -> Result<(), CompilerError> {
        self.visit(expression)?;
        self.context()?.assembler().not(span);

        Ok(())
    }

    fn die_roll(&mut self, expression: SyntaxNodeId, span: Span) -> Result<(), CompilerError> {
        self.context()?.assembler().load_global(DIE_ROLL, span)?;
        self.visit(expression)?;
        self.context()?.assembler().call(1, span);

        Ok(())
    }
}
