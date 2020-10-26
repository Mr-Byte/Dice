use super::NodeVisitor;
use crate::{compiler::Compiler, visitor::FnKind};
use dice_core::{
    protocol::operator::OPERATORS,
    value::{FnScript, Value},
};
use dice_error::compiler_error::CompilerError;
use dice_syntax::OpDecl;

impl NodeVisitor<&OpDecl> for Compiler {
    // TODO: Only allow operators to compile in the context of a prelude?
    fn visit(&mut self, node: &OpDecl) -> Result<(), CompilerError> {
        let body = self.syntax_tree.child(node.body);
        let mut op_context = self.compile_fn(body, &node.args, FnKind::Function)?;
        let name = format!("#{}", node.name);

        if !OPERATORS.with(|ops| ops.contains(&(&*name).into())) {
            return Err(CompilerError::InvalidOperatorName(name, node.span));
        }

        let upvalues = op_context.upvalues().clone();
        let bytecode = op_context.finish();
        let value = Value::FnScript(FnScript::new(name.clone(), bytecode, uuid::Uuid::new_v4()));
        let context = self.context()?;

        emit_bytecode! {
            context.assembler(), node.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST value;
                ] else [
                    CREATE_CLOSURE value, &upvalues;
                ]

                STORE_GLOBAL name;
                PUSH_UNIT;
            ]
        }

        Ok(())
    }
}
