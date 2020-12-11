use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::ScopeKind};
use dice_core::error::{codes::INVALID_CONTINUE_USAGE, Error};
use dice_syntax::Continue;

impl NodeVisitor<&Continue> for Compiler {
    fn visit(&mut self, Continue { span }: &Continue) -> Result<(), Error> {
        let context = self.context()?;
        if !context.scope_stack().in_context_of(ScopeKind::Loop) {
            return Err(Error::new(INVALID_CONTINUE_USAGE).with_span(*span));
        }

        let loop_start = context.scope_stack().entry_point(ScopeKind::Loop)?;
        context.assembler().jump_back(loop_start as u64, *span);

        Ok(())
    }
}
