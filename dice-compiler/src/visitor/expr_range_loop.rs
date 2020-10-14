use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
    visitor::NodeVisitor,
};
use dice_error::{compiler_error::CompilerError, span::Span};
use dice_syntax::{BinaryOperator, ForLoop, SyntaxNode, SyntaxNodeId};

enum RangeLoopKind {
    Exclusive,
    Inclusive,
}

pub(super) struct RangeLoop {
    kind: RangeLoopKind,
    variable: String,
    start: SyntaxNodeId,
    end: SyntaxNodeId,
    body: SyntaxNodeId,
    span: Span,
}

impl NodeVisitor<&RangeLoop> for Compiler {
    fn visit(&mut self, range_loop: &RangeLoop) -> Result<(), CompilerError> {
        // NOTE: Evaluate the end of the range first as a temporary so that it doesn't have to be evaluated again.
        self.visit(range_loop.end)?;
        // NOTE: The start is then evaluated. It will get popped and re-pushed later when stored as a local.
        self.visit(range_loop.start)?;

        let context = self.context()?;
        let loop_start = context.assembler().current_position();
        let loop_exit;

        // NOTE: Start a new scope and define the loop variable.
        context.scope_stack().push_scope(ScopeKind::Loop, None);
        let variable_slot = context
            .scope_stack()
            .add_local(range_loop.variable.clone(), State::initialized(false))? as u8;

        // NOTE: Store the start condition and duplicate the end condition, to be consumed by the loop condition.
        // This effectively reverses the order of the end and start conditions on the stack.
        emit_bytecode! {
            context.assembler(), range_loop.span => [
                STORE_LOCAL variable_slot;
                DUP 1;
                if matches!(range_loop.kind, RangeLoopKind::Exclusive) => [
                    LT;
                ] else [
                    LTE;
                ]
                JUMP_IF_FALSE -> loop_exit;
            ]
        }

        self.visit(range_loop.body)?;

        let context = self.context()?;
        // NOTE: Pop off the value from evaluating the loop body and increment the loop variable by 1.
        emit_bytecode! {
            context.assembler(), range_loop.span => [
                POP;
                LOAD_LOCAL variable_slot;
                PUSH_I1;
                ADD;
            ]
        }

        // NOTE: Close the scope and close out any upvalues at the end of the loop scope, before jumping back.
        let scope_context = context.scope_stack().pop_scope()?;
        emit_bytecode! {
            context.assembler(), range_loop.span => [
                CLOSE_UPVALUES scope_context.variables;
                JUMP_BACK loop_start;
            ]
        }

        // NOTE: Patch all exit points from the loop to jump to after the end of the loop.
        context.assembler().patch_jump(loop_exit);
        for exit_point in scope_context.exit_points {
            context.assembler().patch_jump(exit_point as u64);
        }

        // NOTE: Clean up the temporaries stored on the stack and push a unit value.
        emit_bytecode! {
            context.assembler(), range_loop.span => [
                POP;
                PUSH_UNIT;
            ]
        }
        Ok(())
    }
}

impl Compiler {
    pub(super) fn lower_to_range_loop(&self, for_loop: &ForLoop) -> Option<RangeLoop> {
        let source = self.syntax_tree.get(for_loop.source);

        match source {
            SyntaxNode::Binary(binary) => {
                let kind = match binary.operator {
                    BinaryOperator::RangeInclusive => RangeLoopKind::Inclusive,
                    BinaryOperator::RangeExclusive => RangeLoopKind::Exclusive,
                    _ => return None,
                };

                Some(RangeLoop {
                    kind,
                    variable: for_loop.variable.clone(),
                    start: binary.lhs_expression,
                    end: binary.rhs_expression,
                    body: for_loop.body,
                    span: for_loop.span,
                })
            }
            _ => None,
        }
    }
}
