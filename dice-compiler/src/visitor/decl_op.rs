use super::NodeVisitor;
use crate::{compiler::Compiler, upvalue::UpvalueDescriptor, visitor::FnKind};
use dice_core::{
    error::Error,
    protocol::{
        operator::{
            ADD, DICE_ROLL, DIE_ROLL, DIV, EQ, GT, GTE, LT, LTE, MUL, NEQ, RANGE_EXCLUSIVE, RANGE_INCLUSIVE, REM, SUB,
        },
        ProtocolSymbol,
    },
    value::{FnScript, Symbol, Value},
};
use dice_syntax::{OpDecl, OverloadedOperator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpKind {
    Global,
    Method,
}

impl NodeVisitor<(&OpDecl, OpKind)> for Compiler {
    // TODO: Only allow operators to compile in the context of a prelude?
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
        let bytecode = op_context.finish();
        let value = Value::FnScript(FnScript::new(name.clone(), bytecode, uuid::Uuid::new_v4()));

        match kind {
            OpKind::Global => self.emit_op_global(node, name, &upvalues, value)?,
            OpKind::Method => self.emit_op_method(node, &upvalues, value)?,
        }

        Ok(())
    }
}

impl Compiler {
    pub fn op_name(node: &OpDecl) -> Symbol {
        let name = match node.operator {
            OverloadedOperator::DiceRoll => DICE_ROLL.get(),
            OverloadedOperator::DieRoll => DIE_ROLL.get(),
            OverloadedOperator::Multiply => MUL.get(),
            OverloadedOperator::Divide => DIV.get(),
            OverloadedOperator::Remainder => REM.get(),
            OverloadedOperator::Add => ADD.get(),
            OverloadedOperator::Subtract => SUB.get(),
            OverloadedOperator::GreaterThan => GT.get(),
            OverloadedOperator::LessThan => LT.get(),
            OverloadedOperator::GreaterThanEquals => GTE.get(),
            OverloadedOperator::LessThanEquals => LTE.get(),
            OverloadedOperator::Equals => EQ.get(),
            OverloadedOperator::NotEquals => NEQ.get(),
            OverloadedOperator::RangeInclusive => RANGE_INCLUSIVE.get(),
            OverloadedOperator::RangeExclusive => RANGE_EXCLUSIVE.get(),
        };
        name
    }
}

impl Compiler {
    fn emit_op_global(
        &mut self,
        op_decl: &OpDecl,
        name: Symbol,
        upvalues: &[UpvalueDescriptor],
        compiled_op: Value,
    ) -> Result<(), Error> {
        emit_bytecode! {
            self.assembler()?, op_decl.span => [
                if upvalues.is_empty() => [
                    PUSH_CONST compiled_op;
                ] else [
                    CREATE_CLOSURE compiled_op, &upvalues;
                ]

                STORE_GLOBAL name;
                PUSH_UNIT;
            ]
        }

        Ok(())
    }

    fn emit_op_method(
        &mut self,
        op_decl: &OpDecl,
        upvalues: &[UpvalueDescriptor],
        compiled_op: Value,
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
