use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::ScopeVariable};
use dice_error::compiler_error::CompilerError;
use dice_error::span::Span;
use dice_syntax::{Assignment, AssignmentOperator, FieldAccess, Index, SyntaxNode, SyntaxNodeId};

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
        self.visit(target.expression)?;
        self.visit(target.index_expression)?;

        match assignment.operator {
            AssignmentOperator::Assignment => {
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().store_index(assignment.span);
            }
            operator => {
                self.context()?.assembler().dup(1, assignment.span);
                self.context()?.assembler().dup(1, assignment.span);
                self.context()?.assembler().load_index(assignment.span);
                self.visit(assignment.rhs_expression)?;
                self.visit_operator(operator, assignment.span)?;
                self.context()?.assembler().store_index(assignment.span);
            }
        }

        Ok(())
    }

    fn assign_field(&mut self, target: FieldAccess, assignment: &Assignment) -> Result<(), CompilerError> {
        self.visit(target.expression)?;

        match assignment.operator {
            AssignmentOperator::Assignment => {
                self.visit(assignment.rhs_expression)?;
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
            operator => {
                self.context()?.assembler().dup(0, target.span);
                self.context()?.assembler().load_field(&target.field, target.span)?;
                self.visit(assignment.rhs_expression)?;
                self.visit_operator(operator, target.span)?;
                self.context()?.assembler().store_field(&target.field, target.span)?;
            }
        }

        Ok(())
    }

    fn assign_ident(&mut self, target: String, assignment: &Assignment) -> Result<(), CompilerError> {
        {
            if let Some(local) = self.context()?.scope_stack().local(&target) {
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
            operator => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.visit(rhs_expression)?;
                self.visit_operator(operator, span)?;
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
            operator => {
                self.context()?.assembler().load_local(slot, span);
                self.visit(rhs_expression)?;
                self.visit_operator(operator, span)?;
                self.context()?.assembler().store_local(slot, span);
            }
        }

        Ok(())
    }

    fn visit_operator(&mut self, operator: AssignmentOperator, span: Span) -> Result<(), CompilerError> {
        match operator {
            AssignmentOperator::MulAssignment => self.context()?.assembler().mul(span),
            AssignmentOperator::DivAssignment => self.context()?.assembler().div(span),
            AssignmentOperator::AddAssignment => self.context()?.assembler().add(span),
            AssignmentOperator::SubAssignment => self.context()?.assembler().sub(span),
            _ => unreachable!(),
        }

        Ok(())
    }
}
