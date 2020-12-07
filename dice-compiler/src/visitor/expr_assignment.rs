use super::NodeVisitor;
use crate::{compiler::Compiler, compiler_error::CompilerError, scope_stack::ScopeVariable};
use dice_core::{span::Span, value::Symbol};
use dice_syntax::{Assignment, AssignmentOperator, FieldAccess, Index, SyntaxNode, SyntaxNodeId};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, assignment: &Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(assignment.lhs_expression);

        match lhs {
            SyntaxNode::LitIdent(lit_ident) => {
                let target: Symbol = (&*lit_ident.name).into();
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
            _ => Err(CompilerError::new("Invalid assignment target.", lhs.span())),
        }
    }
}

impl Compiler {
    fn assign_index(&mut self, target: Index, assignment: &Assignment) -> Result<(), CompilerError> {
        self.visit(target.expression)?;
        self.visit(target.index_expression)?;

        match assignment.operator {
            AssignmentOperator::Assignment => {
                emit_bytecode! {
                    self.assembler()?, assignment.span => [
                        {self.visit(assignment.rhs_expression)?};
                        ASSIGN_INDEX;
                    ]
                }
            }
            operator => {
                emit_bytecode! {
                    self.assembler()?, assignment.span => [
                        DUP 1;
                        DUP 1;
                        LOAD_INDEX;
                        {self.visit(assignment.rhs_expression)?};
                        {self.visit_operator(operator, assignment.span)?};
                        ASSIGN_INDEX;
                    ]
                }
            }
        }

        Ok(())
    }

    fn assign_field(&mut self, target: FieldAccess, assignment: &Assignment) -> Result<(), CompilerError> {
        self.visit(target.expression)?;

        match assignment.operator {
            AssignmentOperator::Assignment => {
                emit_bytecode! {
                    self.assembler()?, target.span => [
                        {self.visit(assignment.rhs_expression)?};
                        STORE_FIELD target.field;
                    ]
                }
            }
            operator => {
                emit_bytecode! {
                    self.assembler()?, target.span => [
                        DUP 0;
                        LOAD_FIELD &*target.field;
                        {self.visit(assignment.rhs_expression)?};
                        {self.visit_operator(operator, target.span)?};
                        ASSIGN_FIELD target.field;
                    ]
                }
            }
        }

        Ok(())
    }

    fn assign_ident(&mut self, target: Symbol, assignment: &Assignment) -> Result<(), CompilerError> {
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
                Err(CompilerError::new(
                    format!("No variable with the name {} was declared.", (&*target).to_owned()),
                    assignment.span,
                ))
            }
        }
    }

    fn assign_upvalue(
        &mut self,
        target: Symbol,
        operator: AssignmentOperator,
        rhs_expression: SyntaxNodeId,
        span: Span,
        upvalue: usize,
    ) -> Result<(), CompilerError> {
        if !self.context()?.upvalues()[upvalue].is_mutable() {
            return Err(CompilerError::new(
                format!("Cannot assign to the immutable variable {}.", (&*target).to_owned()),
                span,
            ));
        }
        match operator {
            AssignmentOperator::Assignment => {
                emit_bytecode! {
                    self.assembler()?, span => [
                        {self.visit(rhs_expression)?};
                        ASSIGN_UPVALUE upvalue as u8;
                    ]
                };
            }
            operator => {
                emit_bytecode! {
                    self.assembler()?, span => [
                        LOAD_UPVALUE upvalue as u8;
                        {self.visit(rhs_expression)?};
                        {self.visit_operator(operator, span)?};
                        ASSIGN_UPVALUE upvalue as u8;
                    ]
                };
            }
        }

        Ok(())
    }

    fn assign_local(
        &mut self,
        target: Symbol,
        operator: AssignmentOperator,
        rhs_expression: SyntaxNodeId,
        span: Span,
        local: ScopeVariable,
    ) -> Result<(), CompilerError> {
        let slot = local.slot as u8;

        if !local.is_mutable() {
            return Err(CompilerError::new(
                format!("Cannot assign to the immutable variable {}.", (&*target).to_owned()),
                span,
            ));
        }

        match operator {
            AssignmentOperator::Assignment => {
                emit_bytecode! {
                    self.assembler()?, span => [
                        {self.visit(rhs_expression)?};
                        ASSIGN_LOCAL slot;
                    ]
                }
            }
            operator => {
                emit_bytecode! {
                    self.assembler()?, span => [
                        LOAD_LOCAL slot;
                        {self.visit(rhs_expression)?};
                        {self.visit_operator(operator, span)?};
                        ASSIGN_LOCAL slot;
                    ]
                }
            }
        }

        Ok(())
    }

    fn visit_operator(&mut self, operator: AssignmentOperator, span: Span) -> Result<(), CompilerError> {
        match operator {
            AssignmentOperator::MulAssignment => self.assembler()?.mul(span),
            AssignmentOperator::DivAssignment => self.assembler()?.div(span),
            AssignmentOperator::AddAssignment => self.assembler()?.add(span),
            AssignmentOperator::SubAssignment => self.assembler()?.sub(span),
            _ => unreachable!(),
        }

        Ok(())
    }
}
