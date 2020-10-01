use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::ScopeVariable};
use dice_syntax::{Assignment, AssignmentOperator, FieldAccess, Index, Span, SyntaxNode, SyntaxNodeId};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, assignment: &Assignment) -> Result<(), CompilerError> {
        let lhs = self
            .syntax_tree
            .get(assignment.lhs_expression)
            .expect("Node should exist.");

        match lhs {
            SyntaxNode::LitIdent(lit_ident) => {
                let target = lit_ident.name.clone();
                self.assign_ident(target, assignment)
            }
            SyntaxNode::FieldAccess(field_access) => {
                let field_access = field_access.clone();
                self.assign_field(field_access, assignment)
            }
            SyntaxNode::Index(index) => {
                let index = index.clone();
                self.assign_index(index, assignment)
            }
            _ => Err(CompilerError::InvalidAssignmentTarget),
        }
    }
}

impl Compiler {
    fn assign_index(&mut self, target: Index, assignment: &Assignment) -> Result<(), CompilerError> {
        // todo!()
        // self.visit(target.expression)?;
        //
        // match assignment.operator {
        //     AssignmentOperator::Assignment => {
        //         self.visit(assignment.rhs_expression)?;
        //         self.context()?.assembler().store_field(&target.field, target.span)?;
        //     }
        //     AssignmentOperator::MulAssignment => {
        //         self.context()?.assembler().dup(target.span);
        //         self.context()?.assembler().load_field(&target.field, target.span)?;
        //         self.visit(assignment.rhs_expression)?;
        //         self.context()?.assembler().mul(assignment.span);
        //         self.context()?.assembler().store_field(&target.field, target.span)?;
        //     }
        //     AssignmentOperator::DivAssignment => {
        //         self.context()?.assembler().dup(target.span);
        //         self.context()?.assembler().load_field(&target.field, target.span)?;
        //         self.visit(assignment.rhs_expression)?;
        //         self.context()?.assembler().div(assignment.span);
        //         self.context()?.assembler().store_field(&target.field, target.span)?;
        //     }
        //     AssignmentOperator::AddAssignment => {
        //         self.context()?.assembler().dup(target.span);
        //         self.context()?.assembler().load_field(&target.field, target.span)?;
        //         self.visit(assignment.rhs_expression)?;
        //         self.context()?.assembler().add(assignment.span);
        //         self.context()?.assembler().store_field(&target.field, target.span)?;
        //     }
        //     AssignmentOperator::SubAssignment => {
        //         self.context()?.assembler().dup(target.span);
        //         self.context()?.assembler().load_field(&target.field, target.span)?;
        //         self.visit(assignment.rhs_expression)?;
        //         self.context()?.assembler().sub(assignment.span);
        //         self.context()?.assembler().store_field(&target.field, target.span)?;
        //     }
        // }

        Ok(())
    }

    fn assign_field(&mut self, target: FieldAccess, assignment: &Assignment) -> Result<(), CompilerError> {
        self.visit(target.expression)?;

        match assignment.operator {
            AssignmentOperator::Assignment => {
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
            AssignmentOperator::MulAssignment => {
                self.context()?.assembler().dup(target.span);
                self.context()?.assembler().load_field(&target.field, target.span)?;
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().mul(assignment.span);
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
            AssignmentOperator::DivAssignment => {
                self.context()?.assembler().dup(target.span);
                self.context()?.assembler().load_field(&target.field, target.span)?;
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().div(assignment.span);
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
            AssignmentOperator::AddAssignment => {
                self.context()?.assembler().dup(target.span);
                self.context()?.assembler().load_field(&target.field, target.span)?;
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().add(assignment.span);
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
            AssignmentOperator::SubAssignment => {
                self.context()?.assembler().dup(target.span);
                self.context()?.assembler().load_field(&target.field, target.span)?;
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().sub(assignment.span);
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
        }

        Ok(())
    }

    fn assign_ident(&mut self, target: String, assignment: &Assignment) -> Result<(), CompilerError> {
        {
            if let Some(local) = self.context()?.scope_stack().local(target.clone()) {
                let local = local.clone();
                self.assign_local(
                    target,
                    assignment.operator,
                    assignment.rhs_expression,
                    assignment.span,
                    local,
                )
            } else if let Some(upvalue) = self.compiler_stack.resolve_upvalue(target.clone(), 0) {
                self.assign_upvalue(
                    target,
                    assignment.operator,
                    assignment.rhs_expression,
                    assignment.span,
                    upvalue,
                )
            } else {
                Err(CompilerError::UndeclaredVariable(target))
            }
        }
    }

    fn assign_upvalue(
        &mut self,
        target: String,
        operator: AssignmentOperator,
        rhs_expression: SyntaxNodeId,
        span: Span,
        upvalue: usize,
    ) -> Result<(), CompilerError> {
        if !self.context()?.upvalues()[upvalue].is_mutable() {
            return Err(CompilerError::ImmutableVariable(target));
        }
        match operator {
            AssignmentOperator::Assignment => {
                self.visit(rhs_expression)?;
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::MulAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().mul(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::DivAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().div(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::AddAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().add(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::SubAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().sub(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
        }

        Ok(())
    }

    fn assign_local(
        &mut self,
        target: String,
        operator: AssignmentOperator,
        rhs_expression: SyntaxNodeId,
        span: Span,
        local: ScopeVariable,
    ) -> Result<(), CompilerError> {
        let slot = local.slot as u8;

        if !local.is_mutable() {
            return Err(CompilerError::ImmutableVariable(target));
        }

        match operator {
            AssignmentOperator::Assignment => {
                self.visit(rhs_expression)?;
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::MulAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().mul(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::DivAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().div(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::AddAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().add(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::SubAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.visit(rhs_expression)?;
                self.context()?.assembler().sub(span);
                self.context()?.assembler().store_local(slot, span);
            }
        }

        Ok(())
    }
}
