use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::span::Span;
use dice_error::compiler_error::CompilerError;
use dice_syntax::{SyntaxNodeId, Unary, UnaryOperator};

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
        self.assembler()?.neg(span);

        Ok(())
    }

    fn not(&mut self, expression: SyntaxNodeId, span: Span) -> Result<(), CompilerError> {
        self.visit(expression)?;
        self.assembler()?.not(span);

        Ok(())
    }

    fn die_roll(&mut self, expression: SyntaxNodeId, span: Span) -> Result<(), CompilerError> {
        self.visit(expression)?;
        self.assembler()?.die_roll(span);

        Ok(())
    }
}
