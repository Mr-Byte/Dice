use crate::compiler::Compiler;
use crate::error::CompilerError;
use crate::scope_stack::{ScopeKind, State};
use crate::visitor::NodeVisitor;
use dice_syntax::{BinaryOperator, ForLoop, Span, SyntaxNode, SyntaxNodeId};

pub(super) enum RangeLoopKind {
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
        // NOTE: Evaluate the end of the range first and duplicate it, so that it doesn't have to be evaluated again.
        // This means that the loop condition will need to be > for exclusive, and >= for inclusive.
        self.visit(range_loop.end)?;
        self.visit(range_loop.start)?;

        let context = self.context()?;
        let loop_start = context.assembler().current_position();
        let loop_exit;

        // NOTE: Duplicate the end and start conditions, to be consumed by the loop condition.
        emit_bytecode! {
            context.assembler(), range_loop.span =>
                DUP 1;
                DUP 1;
        }

        match range_loop.kind {
            RangeLoopKind::Exclusive => emit_bytecode! { context.assembler(), range_loop.span => GT; },
            RangeLoopKind::Inclusive => emit_bytecode! { context.assembler(), range_loop.span => GTE; },
        };

        emit_bytecode! {
            context.assembler(), range_loop.span =>
                loop_exit = JUMP_IF_FALSE;
        }

        context.scope_stack().push_scope(ScopeKind::Loop, None);

        let variable_slot = context.scope_stack().add_local(
            range_loop.variable.clone(),
            State::Local {
                is_mutable: false,
                is_initialized: true,
            },
        )? as u8;

        // NOTE: Store the current value at the top of the stack in the loop variable.
        emit_bytecode! {
            context.assembler(), range_loop.span =>
                STORE_LOCAL variable_slot;
                POP;
        }

        self.visit(range_loop.body)?;

        let context = self.context()?;
        emit_bytecode! {
            context.assembler(), range_loop.span =>
                POP;
                LOAD_LOCAL variable_slot;
                PUSH_I1;
                ADD;
        }

        let scope_context = context.scope_stack().pop_scope()?;

        emit_bytecode! {
            context.assembler(), range_loop.span =>
                CLOSE_UPVALUES scope_context.variables;
                JUMP_BACK loop_start;
        }

        context.assembler().patch_jump(loop_exit);
        for exit_point in scope_context.exit_points {
            context.assembler().patch_jump(exit_point as u64);
        }

        // NOTE: Clean up the temporaries stored on the stack and push a unit value.
        emit_bytecode! {
            context.assembler(), range_loop.span =>
                POP;
                POP;
                PUSH_UNIT;
        }
        Ok(())
    }
}

impl Compiler {
    pub(super) fn lower_range_loop(&self, for_loop: &ForLoop) -> Option<RangeLoop> {
        let source = self.syntax_tree.get(for_loop.source).expect("Node should exist.");

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
