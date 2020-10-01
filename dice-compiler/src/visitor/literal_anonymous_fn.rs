use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError, scope_stack::State};
use dice_core::value::{FnScript, Value};
use dice_syntax::LitAnonymousFn;

// TODO: Extract shared code between anonymous and named fns into a common visitor.
impl NodeVisitor<&LitAnonymousFn> for Compiler {
    fn visit(&mut self, node: &LitAnonymousFn) -> Result<(), CompilerError> {
        let id = uuid::Uuid::new_v4();
        let name = format!("__anonymous_fn_{:X}", id.clone().to_simple());
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut fn_context = self.compile_fn(body, &node.args)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish();
        let value = Value::FnScript(FnScript::new(name, node.args.len(), bytecode, id));
        let context = self.context()?;

        if !upvalues.is_empty() {
            context.assembler().closure(value, &upvalues, node.span)?;
        } else {
            context.assembler().push_const(value, node.span)?;
        }

        Ok(())
    }
}
