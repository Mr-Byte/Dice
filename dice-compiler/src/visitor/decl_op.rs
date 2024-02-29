use dice_bytecode::{ConstantValue, FunctionBytecode};
use dice_core::{
    error::Error,
    protocol::operator::{ADD, DIV, EQ, GT, GTE, LT, LTE, MUL, NEQ, RANGE_EXCLUSIVE, RANGE_INCLUSIVE, REM, SUB},
};
use dice_syntax::{OpDecl, OverloadedOperator};

use crate::{compiler::Compiler, upvalue::UpvalueDescriptor, visitor::FnKind};

use super::NodeVisitor;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpKind {
    Global,
    Method,
}

impl NodeVisitor<(&OpDecl, OpKind)> for Compiler {
    fn visit(&mut self, (node, kind): (&OpDecl, OpKind)) -> Result<(), Error> {
        Self::assert_unique_params(&node.args, node.span)?;

        let body = self.syntax_tree.child(node.body);
        let fn_kind = match kind {
            OpKind::Global => FnKind::Function,
            OpKind::Method => FnKind::Method,
        };
        let mut op_context = self.compile_fn(body, &node.args, node.return_.clone(), fn_kind)?;
        let name = Compiler::op_name(node);

        let upvalues = op_context.upvalues().clone();
        let bytecode = op_context.finish(self.source.clone());
        let value = ConstantValue::Function(FunctionBytecode::new(bytecode, name, uuid::Uuid::new_v4()));

        match kind {
            OpKind::Global => self.emit_op_global(node, name, &upvalues, value)?,
            OpKind::Method => self.emit_op_method(node, &upvalues, value)?,
        }

        Ok(())
    }
}

impl Compiler {
    pub fn op_name(node: &OpDecl) -> &str {
        let name = match node.operator {
            OverloadedOperator::Multiply => MUL,
            OverloadedOperator::Divide => DIV,
            OverloadedOperator::Remainder => REM,
            OverloadedOperator::Add => ADD,
            OverloadedOperator::Subtract => SUB,
            OverloadedOperator::GreaterThan => GT,
            OverloadedOperator::LessThan => LT,
            OverloadedOperator::GreaterThanEquals => GTE,
            OverloadedOperator::LessThanEquals => LTE,
            OverloadedOperator::Equals => EQ,
            OverloadedOperator::NotEquals => NEQ,
            OverloadedOperator::RangeInclusive => RANGE_INCLUSIVE,
            OverloadedOperator::RangeExclusive => RANGE_EXCLUSIVE,
        };
        name
    }
}

impl Compiler {
    fn emit_op_global(
        &mut self,
        op_decl: &OpDecl,
        name: impl Into<String>,
        upvalues: &[UpvalueDescriptor],
        compiled_op: ConstantValue,
    ) -> Result<(), Error> {
        emit_bytecode! {
            self.assembler()?, op_decl.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST compiled_op;
                ] else [
                    CREATE_CLOSURE compiled_op, &upvalues;
                ]

                STORE_GLOBAL name.into();
                PUSH_UNIT;
            ]
        }

        Ok(())
    }

    fn emit_op_method(
        &mut self,
        op_decl: &OpDecl,
        upvalues: &[UpvalueDescriptor],
        compiled_op: ConstantValue,
    ) -> Result<(), Error> {
        emit_bytecode! {
            self.assembler()?, op_decl.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST compiled_op;
                ] else [
                    CREATE_CLOSURE compiled_op, upvalues;
                ]
            ]
        }

        Ok(())
    }
}
