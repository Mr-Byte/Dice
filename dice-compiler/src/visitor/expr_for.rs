use super::NodeVisitor;
use crate::compiler_error::CompilerError;
use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
};
use dice_core::protocol::iterator::{DONE, ITER, NEXT, VALUE};
use dice_syntax::ForLoop;

impl NodeVisitor<&ForLoop> for Compiler {
    fn visit(&mut self, for_loop: &ForLoop) -> Result<(), CompilerError> {
        if let Some(range_loop) = self.lower_to_range_loop(for_loop) {
            return self.visit(&range_loop);
        }

        // NOTE: Visit the source first.
        self.visit(for_loop.source)?;
        // NEXT: Load the iterator method, call it, and load the next method from it.
        emit_bytecode! {
            self.assembler()?, for_loop.span => [
                LOAD_FIELD &ITER;
                CALL 0;
                LOAD_FIELD &NEXT;
            ]
        };

        let context = self.context()?;
        let loop_start = context.assembler().current_position();
        let loop_exit;

        // NOTE: Start a new scope and define the loop variable.
        context.scope_stack().push_scope(ScopeKind::Loop, None);
        let variable_slot = context
            .scope_stack()
            .add_local(for_loop.variable.clone(), State::initialized(false))? as u8;

        emit_bytecode! {
            context.assembler(), for_loop.span => [
                DUP 0;
                CALL 0;
                DUP 0;
                LOAD_FIELD &DONE;
                JUMP_IF_TRUE -> loop_exit;
                LOAD_FIELD_TO_LOCAL &VALUE, variable_slot;
                POP;
            ]
        }

        self.visit(for_loop.body)?;

        let context = self.context()?;

        // NOTE: Close the scope and close out any upvalues at the end of the loop scope, before jumping back.
        let scope_context = context.scope_stack().pop_scope()?;
        emit_bytecode! {
            context.assembler(), for_loop.span => [
                POP;
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
            context.assembler(), for_loop.span => [
                POP;
                POP;
                PUSH_UNIT;
            ]
        }

        Ok(())
    }
}
