use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_core::value::{FnScript, Value};
use dice_syntax::OpDecl;

impl NodeVisitor<&OpDecl> for Compiler {
    fn visit(&mut self, node: &OpDecl) -> Result<(), CompilerError> {
        // TODO: Enforce operator names.

        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut op_context = self.compile_fn(body, &node.args)?;
        let name = format!("#{}", node.name);
        let upvalues = op_context.upvalues().clone();
        let bytecode = op_context.finish();
        let value = Value::FnScript(FnScript::new(
            name.clone(),
            node.args.len(),
            bytecode,
            uuid::Uuid::new_v4(),
        ));
        let context = self.context()?;

        if !upvalues.is_empty() {
            context.assembler().closure(value, &upvalues, node.span)?;
        } else {
            context.assembler().push_const(value, node.span)?;
        }

        context.assembler().store_global(Value::new_string(name), node.span)?;
        // NOTE: Operators exist as temporaries and should produce a unit on the stack.
        context.assembler().push_unit(node.span);

        Ok(())
    }
}
