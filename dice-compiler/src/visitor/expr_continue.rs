use super::NodeVisitor;
use crate::{compiler::Compiler, scope_stack::ScopeKind};
use dice_error::compiler_error::CompilerError;
use dice_syntax::Continue;

impl NodeVisitor<&Continue> for Compiler {
    fn visit(&mut self, Continue { span }: &Continue) -> Result<(), CompilerError> {
        let context = self.context()?;
        if !context.scope_stack().in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidContinue);
        }

        let loop_start = context.scope_stack().entry_point(ScopeKind::Loop)?;
        context.assembler().jump_back(loop_start as u64, *span);

        Ok(())
    }
}
