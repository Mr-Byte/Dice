use super::NodeVisitor;
use crate::compiler::Compiler;
use dice_core::{error::Error, span::Span};
use dice_syntax::{Binary, BinaryOperator, SyntaxNodeId};

impl NodeVisitor<&Binary> for Compiler {
    fn visit(
        &mut self,
        Binary {
            operator,
            lhs_expression,
            rhs_expression,
            span,
        }: &Binary,
    ) -> Result<(), Error> {
        match operator {
            BinaryOperator::LogicalAnd => self.logical_and(*lhs_expression, *rhs_expression, *span)?,
            BinaryOperator::LogicalOr => self.logical_or(*lhs_expression, *rhs_expression, *span)?,
            BinaryOperator::Pipeline => self.pipeline(*lhs_expression, *rhs_expression, *span)?,
            BinaryOperator::Coalesce => self.coalesce(*lhs_expression, *rhs_expression, *span)?,
            operator => self.binary(*operator, *lhs_expression, *rhs_expression, *span)?,
        }

        Ok(())
    }
}

impl Compiler {
    fn logical_and(&mut self, lhs_expression: SyntaxNodeId, rhs_expression: SyntaxNodeId, span: Span) -> Result<(), Error> {
        let short_circuit_jump;

        emit_bytecode! {
            self.assembler()?, span => [
                { self.visit(lhs_expression)? };
                DUP 0;
                ASSERT_BOOL;
                JUMP_IF_FALSE -> short_circuit_jump;
                POP;
                { self.visit(rhs_expression)? };
                ASSERT_BOOL;
            ]
        }

        self.compiler_stack.top_mut()?.assembler().patch_jump(short_circuit_jump);

        Ok(())
    }
}

impl Compiler {
    fn logical_or(&mut self, lhs_expression: SyntaxNodeId, rhs_expression: SyntaxNodeId, span: Span) -> Result<(), Error> {
        let short_circuit_jump;

        emit_bytecode! {
            self.assembler()?, span => [
                { self.visit(lhs_expression)? };
                DUP 0;
                ASSERT_BOOL;
                JUMP_IF_TRUE -> short_circuit_jump;
                POP;
                { self.visit(rhs_expression)? };
                ASSERT_BOOL;
            ]
        }

        self.compiler_stack.top_mut()?.assembler().patch_jump(short_circuit_jump);

        Ok(())
    }

    fn pipeline(&mut self, lhs_expression: SyntaxNodeId, rhs_expression: SyntaxNodeId, span: Span) -> Result<(), Error> {
        self.visit(rhs_expression)?;
        self.visit(lhs_expression)?;
        self.assembler()?.call(1, span);

        Ok(())
    }

    fn coalesce(&mut self, lhs_expression: SyntaxNodeId, rhs_expression: SyntaxNodeId, span: Span) -> Result<(), Error> {
        self.visit(lhs_expression)?;
        let coalesce_jump;
        emit_bytecode! {
            self.assembler()?, span => [
                DUP 0;
                PUSH_NULL;
                EQ;
                JUMP_IF_FALSE -> coalesce_jump;
                POP;
            ]
        }

        self.visit(rhs_expression)?;
        self.assembler()?.patch_jump(coalesce_jump);

        Ok(())
    }

    fn binary(&mut self, operator: BinaryOperator, lhs_expression: SyntaxNodeId, rhs_expression: SyntaxNodeId, span: Span) -> Result<(), Error> {
        self.visit(lhs_expression)?;
        self.visit(rhs_expression)?;

        match operator {
            BinaryOperator::Multiply => self.assembler()?.mul(span),
            BinaryOperator::Divide => self.assembler()?.div(span),
            BinaryOperator::Remainder => self.assembler()?.rem(span),
            BinaryOperator::Add => self.assembler()?.add(span),
            BinaryOperator::Subtract => self.assembler()?.sub(span),
            BinaryOperator::GreaterThan => self.assembler()?.gt(span),
            BinaryOperator::LessThan => self.assembler()?.lt(span),
            BinaryOperator::GreaterThanEquals => self.assembler()?.gte(span),
            BinaryOperator::LessThanEquals => self.assembler()?.lte(span),
            BinaryOperator::Equals => self.assembler()?.eq(span),
            BinaryOperator::NotEquals => self.assembler()?.neq(span),
            // BinaryOperator::Is => self.assembler()?.is(span),
            BinaryOperator::DiceRoll => self.assembler()?.dice_roll(span),
            BinaryOperator::RangeInclusive => self.assembler()?.range_inclusive(span),
            BinaryOperator::RangeExclusive => self.assembler()?.range_exclusive(span),
            _ => unreachable!(),
        }

        Ok(())
    }
}
