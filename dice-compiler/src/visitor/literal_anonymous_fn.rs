use super::NodeVisitor;
use crate::{compiler::Compiler, visitor::FnKind};
use dice_core::{
    error::Error,
    value::{FnScript, Value},
};
use dice_syntax::LitAnonymousFn;

impl NodeVisitor<&LitAnonymousFn> for Compiler {
    fn visit(&mut self, node: &LitAnonymousFn) -> Result<(), Error> {
        let id = uuid::Uuid::new_v4();
        let name = format!("__anonymous_fn_{id}");
        let body = self.syntax_tree.child(node.body);
        let mut fn_context = self.compile_fn(body, &node.args, node.return_.clone(), FnKind::Function)?;
        let upvalues = fn_context.upvalues().clone();
        let bytecode = fn_context.finish(self.source.clone());
        let value = Value::FnScript(FnScript::new(name, bytecode, id));
        let context = self.context()?;

        if upvalues.is_empty() {
            context.assembler().push_const(value, node.span)?;
        } else {
            context.assembler().closure(value, &upvalues, node.span)?;
        }

        Ok(())
    }
}
