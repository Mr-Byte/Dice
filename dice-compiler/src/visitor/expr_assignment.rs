use super::NodeVisitor;
use crate::compiler::Compiler;
use crate::error::CompilerError;
use crate::scope_stack::ScopeVariable;
use dice_syntax::{Assignment, AssignmentOperator, Span, SyntaxNode, SyntaxNodeId};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, assignment: &Assignment) -> Result<(), CompilerError> {
        let lhs = self
            .syntax_tree
            .get(assignment.lhs_expression)
            .expect("Node should exist.");

        // TODO: Decompose this down into smaller functions.
        match lhs {
            SyntaxNode::LitIdent(lit_ident) => {
                let target = lit_ident.name.clone();
                self.assign_ident(target, assignment)
            }
            SyntaxNode::FieldAccess(_field_access) => todo!("Field assignment not implemented"),
            _ => Err(CompilerError::InvalidAssignmentTarget),
        }
    }
}

impl Compiler {
    fn assign_ident(
        &mut self,
        target: String,
        Assignment {
            operator,
            rhs_expression,
            span,
            ..
        }: &Assignment,
    ) -> Result<(), CompilerError> {
        {
            if let Some(local) = self.context()?.scope_stack().local(target.clone()) {
                let local = local.clone();
                self.assign_local(target, *operator, *rhs_expression, *span, local)
            } else if let Some(upvalue) = self.compiler_stack.resolve_upvalue(target.clone(), 0) {
                self.assign_upvalue(target, *operator, *rhs_expression, *span, upvalue)
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

        self.visit(rhs_expression)?;

        match operator {
            AssignmentOperator::Assignment => {
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::MulAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.context()?.assembler().mul(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::DivAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.context()?.assembler().div(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::AddAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.context()?.assembler().add(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
            AssignmentOperator::SubAssignment => {
                self.context()?.assembler().load_upvalue(upvalue as u8, span);
                self.context()?.assembler().sub(span);
                self.context()?.assembler().store_upvalue(upvalue as u8, span);
            }
        }

        return Ok(());
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

        self.visit(rhs_expression)?;

        match operator {
            AssignmentOperator::Assignment => {
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::MulAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.context()?.assembler().mul(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::DivAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.context()?.assembler().div(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::AddAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.context()?.assembler().add(span);
                self.context()?.assembler().store_local(slot, span);
            }
            AssignmentOperator::SubAssignment => {
                self.context()?.assembler().load_local(slot, span);
                self.context()?.assembler().sub(span);
                self.context()?.assembler().store_local(slot, span);
            }
        }

        return Ok(());
    }
}
