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
        // NOTE: Evaluate the end range first and duplicate it, so that it doesn't have to be evaluated again.
        // This means that the loop condition will need to be > for exclusive, and >= for inclusive.
        self.visit(range_loop.end)?;
        self.visit(range_loop.start)?;
        self.context()?.assembler().dup(1, range_loop.span);
        self.context()?.assembler().dup(1, range_loop.span);

        let loop_start = self.context()?.assembler().current_position();
        match range_loop.kind {
            RangeLoopKind::Exclusive => self.context()?.assembler().gt(range_loop.span),
            RangeLoopKind::Inclusive => self.context()?.assembler().gte(range_loop.span),
        }
        let loop_jump = self.context()?.assembler().jump_if_false(range_loop.span);

        self.context()?.scope_stack().push_scope(ScopeKind::Loop, None);

        let variable_slot = self.context()?.scope_stack().add_local(
            range_loop.variable.clone(),
            State::Local {
                is_mutable: false,
                is_initialized: true,
            },
        )?;

        // NOTE: Store the current value at the top of the stack in the loop variable.
        self.context()?
            .assembler()
            .store_local(variable_slot as u8, range_loop.span);
        self.context()?.assembler().pop(range_loop.span);

        self.visit(range_loop.body)?;
        self.context()?.assembler().pop(range_loop.span);

        self.context()?
            .assembler()
            .load_local(variable_slot as u8, range_loop.span);
        self.context()?.assembler().push_i1(range_loop.span);
        self.context()?.assembler().add(range_loop.span);
        self.context()?.assembler().dup(1, range_loop.span);
        self.context()?.assembler().dup(1, range_loop.span);

        let scope_context = self.context()?.scope_stack().pop_scope()?;
        self.context()?.assembler().jump_back(loop_start, range_loop.span);

        self.context()?.assembler().patch_jump(loop_jump);
        for exit_point in scope_context.exit_points {
            self.context()?.assembler().patch_jump(exit_point as u64);
        }

        // NOTE: Clean up the temporaries stored on the stack.
        self.context()?.assembler().pop(range_loop.span);
        self.context()?.assembler().pop(range_loop.span);

        // NOTE: For loops always evaluate to Unit.
        self.context()?.assembler().push_unit(range_loop.span);

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
