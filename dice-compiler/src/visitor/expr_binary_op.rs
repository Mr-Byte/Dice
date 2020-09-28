use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_syntax::{Binary, BinaryOperator};

impl NodeVisitor<&Binary> for Compiler {
    fn visit(
        &mut self,
        Binary {
            operator,
            lhs_expression,
            rhs_expression,
            span,
        }: &Binary,
    ) -> Result<(), CompilerError> {
        // TODO: Decompose this into multiple expressions.
        match operator {
            BinaryOperator::LogicalAnd => {
                self.visit(*lhs_expression)?;
                self.context()?.assembler().dup(*span);

                let short_circuit_jump = self.context()?.assembler().jump_if_false(*span);
                self.context()?.assembler().pop(*span);
                self.visit(*rhs_expression)?;
                self.context()?.assembler().assert_bool(*span);

                self.compiler_stack
                    .top_mut()?
                    .assembler()
                    .patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.visit(*lhs_expression)?;
                self.context()?.assembler().dup(*span);
                self.context()?.assembler().not(*span);

                let short_circuit_jump = self.context()?.assembler().jump_if_false(*span);
                self.context()?.assembler().pop(*span);
                self.visit(*rhs_expression)?;
                self.context()?.assembler().assert_bool(*span);

                self.compiler_stack
                    .top_mut()?
                    .assembler()
                    .patch_jump(short_circuit_jump);
            }
            _ => {
                self.visit(*lhs_expression)?;
                self.visit(*rhs_expression)?;

                match operator {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.context()?.assembler().mul(*span),
                    BinaryOperator::Divide => self.context()?.assembler().div(*span),
                    BinaryOperator::Remainder => self.context()?.assembler().rem(*span),
                    BinaryOperator::Add => self.context()?.assembler().add(*span),
                    BinaryOperator::Subtract => self.context()?.assembler().sub(*span),
                    BinaryOperator::GreaterThan => self.context()?.assembler().gt(*span),
                    BinaryOperator::LessThan => self.context()?.assembler().lt(*span),
                    BinaryOperator::GreaterThanEquals => self.context()?.assembler().gte(*span),
                    BinaryOperator::LessThanEquals => self.context()?.assembler().lte(*span),
                    BinaryOperator::Equals => self.context()?.assembler().eq(*span),
                    BinaryOperator::NotEquals => self.context()?.assembler().neq(*span),
                    BinaryOperator::RangeInclusive => todo!(),
                    BinaryOperator::RangeExclusive => todo!(),
                    BinaryOperator::Coalesce => todo!(),
                    _ => unreachable!(),
                }
            }
        }

        Ok(())
    }
}
