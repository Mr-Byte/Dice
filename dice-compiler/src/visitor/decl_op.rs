use super::NodeVisitor;
use crate::{compiler::Compiler, error::CompilerError};
use dice_core::{
    constants::OPERATORS,
    value::{FnScript, Value},
};
use dice_syntax::OpDecl;

impl NodeVisitor<&OpDecl> for Compiler {
    // TODO: Only allow operators to compile in the context of a prelude?
    fn visit(&mut self, node: &OpDecl) -> Result<(), CompilerError> {
        let body = self.syntax_tree.child(node.body).expect("Node should not be missing.");
        let mut op_context = self.compile_fn(body, &node.args)?;
        let name = format!("#{}", node.name);

        if !OPERATORS.contains(&&*name) {
            return Err(CompilerError::InvalidOperatorName(name, node.span));
        }

        let upvalues = op_context.upvalues().clone();
        let bytecode = op_context.finish();
        let value = Value::FnScript(FnScript::new(
            name.clone(),
            node.args.len(),
            bytecode,
            uuid::Uuid::new_v4(),
        ));
        let context = self.context()?;

        emit_bytecode! {
            context.assembler(), node.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST value;
                ] else [
                    CLOSURE value, &upvalues;
                ]

                STORE_GLOBAL &name;
                PUSH_UNIT;
            ]
        }

        Ok(())
    }
}
