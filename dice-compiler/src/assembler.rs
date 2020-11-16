use super::upvalue::UpvalueDescriptor;
use bytes::BufMut as _;
use dice_core::{
    bytecode::{instruction::Instruction, Bytecode},
    value::{Symbol, Value},
};
use dice_error::{compiler_error::CompilerError, span::Span};
use std::collections::HashMap;

#[derive(Default)]
pub struct Assembler {
    constants: Vec<Value>,
    source_map: HashMap<u64, Span>,
    data: Vec<u8>,
}

impl Assembler {
    pub fn generate(self, slot_count: usize, upvalue_count: usize) -> Bytecode {
        Bytecode::new(
            self.data.into(),
            slot_count,
            upvalue_count,
            self.constants.into_boxed_slice(),
            self.source_map,
        )
    }

    pub fn push_null(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushNull.into());
    }

    pub fn push_unit(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushUnit.into());
    }

    pub fn push_bool(&mut self, into: bool, span: Span) {
        let instruction = if into {
            Instruction::PushTrue
        } else {
            Instruction::PushFalse
        };

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(instruction.into());
    }

    pub fn push_i0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushI0.into());
    }

    pub fn push_i1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushI1.into());
    }

    pub fn push_f0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushF0.into());
    }

    pub fn push_f1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushF1.into());
    }

    pub fn push_const(&mut self, into: Value, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PushConst.into());

        self.source_map.insert(self.data.len() as u64, span);
        let const_pos = self.make_constant(into)?;
        self.data.put_u8(const_pos);

        Ok(())
    }

    pub fn closure(&mut self, into: Value, upvalues: &[UpvalueDescriptor], span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CreateClosure.into());
        let fn_pos = self.make_constant(into)?;
        self.data.put_u8(fn_pos);

        if upvalues.len() > 255 {
            return Err(CompilerError::TooManyUpvalues);
        }

        for upvalue in upvalues {
            let (is_parent_local, index) = upvalue.description();

            self.data.put_u8(is_parent_local as u8);
            self.data.put_u8(index as u8);
        }

        Ok(())
    }

    pub fn pop(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Pop.into());
    }

    pub fn dup(&mut self, offset: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Dup.into());
        self.data.put_u8(offset);
    }

    pub fn swap(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Swap.into());
    }

    pub fn create_list(&mut self, length: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CreateArray.into());
        self.data.put_u8(length);
    }

    pub fn create_object(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CreateObject.into());
    }

    pub fn create_class(&mut self, name: &str, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CreateClass.into());
        let name_slot = self.make_constant(Value::with_symbol(name))? as u8;
        self.data.put_u8(name_slot);

        Ok(())
    }

    pub fn inherit_class(&mut self, name: &str, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::InheritClass.into());
        let name_slot = self.make_constant(Value::with_symbol(name))? as u8;
        self.data.put_u8(name_slot);

        Ok(())
    }

    pub fn mul(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Multiply.into());
    }

    pub fn div(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Divide.into());
    }

    pub fn rem(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Remainder.into());
    }

    pub fn add(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Add.into());
    }

    pub fn sub(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Subtract.into());
    }

    pub fn eq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Equal.into());
    }

    pub fn neq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NotEqual.into());
    }

    pub fn is(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Is.into());
    }

    pub fn gt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GreaterThan.into());
    }

    pub fn gte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GreaterThanOrEqual.into());
    }

    pub fn lt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LessThan.into());
    }

    pub fn lte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LessThanOrEqual.into());
    }

    pub fn dice_roll(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DiceRoll.into());
    }

    pub fn range_inclusive(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RangeInclusive.into());
    }

    pub fn range_exclusive(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RangeExclusive.into());
    }

    pub fn die_roll(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DieRoll.into());
    }

    pub fn neg(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Negate.into());
    }

    pub fn not(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Not.into());
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Jump.into());
        let patch_pos = self.data.len() as u64;
        self.data.put_i16(0);

        patch_pos
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump_if_false(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JumpIfFalse.into());
        let patch_pos = self.data.len() as u64;
        self.data.put_i16(0);

        patch_pos
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump_if_true(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JumpIfTrue.into());
        let patch_pos = self.data.len() as u64;
        self.data.put_i16(0);

        patch_pos
    }

    pub fn patch_jump(&mut self, jump_position: u64) {
        let offset = (self.current_position() - jump_position - 2) as i16;
        (&mut self.data[jump_position as usize..]).put_i16(offset)
    }

    pub fn jump_back(&mut self, position: u64, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Jump.into());
        let offset = -((self.current_position() - position + 2) as i16);
        self.data.put_i16(offset);
    }

    pub fn current_position(&self) -> u64 {
        (self.data.len()) as u64
    }

    pub fn store_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::StoreLocal.into());
        self.data.put_u8(slot);
    }

    pub fn assign_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssignLocal.into());
        self.data.put_u8(slot);
    }

    pub fn load_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LoadLocal.into());
        self.data.put_u8(slot);
    }

    #[allow(dead_code)]
    pub fn store_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::StoreUpvalue.into());
        self.data.put_u8(index);
    }

    pub fn assign_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssignUpvalue.into());
        self.data.put_u8(index);
    }

    pub fn load_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LoadUpvalue.into());
        self.data.put_u8(index);
    }

    pub fn close_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CloseUpvalue.into());
        self.data.put_u8(index);
    }

    pub fn store_field(&mut self, field: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::StoreField.into());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn assign_field(&mut self, field: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::AssignField.into());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn load_method(&mut self, method: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(method.into()))?;
        self.data.put_u8(Instruction::LoadMethod.into());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn store_method(&mut self, method: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(method.into()))?;
        self.data.put_u8(Instruction::StoreMethod.into());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn load_field(&mut self, field: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::LoadField.into());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn store_global(&mut self, global: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn store_global_impl(assembler: &mut Assembler, global: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(global))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::StoreGlobal.into());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        store_global_impl(self, global.into(), span)
    }

    pub fn load_global(&mut self, global: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn load_global_impl(assembler: &mut Assembler, global: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(global))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::LoadGlobal.into());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        load_global_impl(self, global.into(), span)
    }

    pub fn load_module(&mut self, path: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn load_module_impl(assembler: &mut Assembler, path: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(path))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::LoadModule.into());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        load_module_impl(self, path.into(), span)
    }

    pub fn load_index(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LoadIndex.into());
    }

    #[allow(dead_code)]
    pub fn store_index(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::StoreIndex.into());
    }

    pub fn assign_index(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssignIndex.into());
    }

    pub fn load_field_to_local(
        &mut self,
        field: impl Into<Symbol>,
        local_slot: u8,
        span: Span,
    ) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::LoadFieldToLocal.into());
        self.data.put_u8(const_slot);
        self.data.put_u8(local_slot);

        Ok(())
    }

    pub fn call(&mut self, arg_count: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Call.into());
        self.data.put_u8(arg_count);
    }

    pub fn ret(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::Return.into());
    }

    pub fn assert_bool(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssertBool.into());
    }

    pub fn assert_type_for_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssertTypeForLocal.into());
        self.data.put_u8(slot);
    }

    pub fn assert_type_or_null_for_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssertTypeOrNullForLocal.into());
        self.data.put_u8(slot);
    }

    pub fn assert_type_and_return(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssertTypeAndReturn.into());
    }

    pub fn assert_type_or_null_and_return(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::AssertTypeOrNullAndReturn.into());
    }

    fn make_constant(&mut self, into: Value) -> Result<u8, CompilerError> {
        let position = if let Some(position) = self.constants.iter().position(|current| *current == into) {
            position
        } else {
            self.constants.push(into);
            self.constants.len() - 1
        };

        // NOTE: This could be alleviated by offering a long-form PushConst.
        if position > 255 {
            return Err(CompilerError::TooManyConstants);
        }

        Ok(position as u8)
    }
}

#[macro_export]
macro_rules! emit_bytecode {
    ($assembler:expr, $span:expr => [] ) => {};

    ($assembler:expr, $span:expr => [if $c:expr => [ $($t:tt)* ] else [ $($f:tt)* ] $($rest:tt)*]) => {
        if $c {
            emit_bytecode! { $assembler, $span => [$($t)*] }
        } else {
            emit_bytecode! { $assembler, $span => [$($f)*] }
        }

        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [if $c:expr => [ $($t:tt)* ] $($rest:tt)*]) => {
        if $c {
            emit_bytecode! { $assembler, $span => [$($t)*] }
        }

        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [if let $p:path = $c:expr => [ $($t:tt)* ] $($rest:tt)*]) => {
        if let $p = $c {
            emit_bytecode! { $assembler, $span => [$($t)*] }
        }

        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [for $i:pat in $c:expr => [ $($b:tt)* ] $($rest:tt)*]) => {
        for $i in $c {
            emit_bytecode! { $assembler, $span => [ $($b)* ]}
        }

        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [PUSH_NULL; $($rest:tt)*] ) => {
        $assembler.push_null($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [PUSH_UNIT; $($rest:tt)*] ) => {
        $assembler.push_unit($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [PUSH_I1; $($rest:tt)*] ) => {
        $assembler.push_i1($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
    ($assembler:expr, $span:expr => [PUSH_BOOL $into:expr; $($rest:tt)*] ) => {
        $assembler.push_bool($into, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [PUSH_CONST $into:expr; $($rest:tt)*] ) => {
        $assembler.push_const($into, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [CREATE_CLOSURE $into:expr, $upvalues:expr; $($rest:tt)*] ) => {
        $assembler.closure($into, $upvalues, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [CREATE_CLASS $name:expr; $($rest:tt)*] ) => {
        $assembler.create_class($name, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [INHERIT_CLASS $name:expr; $($rest:tt)*] ) => {
        $assembler.inherit_class($name, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [POP; $($rest:tt)*] ) => {
        $assembler.pop($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [SWAP; $($rest:tt)*] ) => {
        $assembler.swap($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [DUP $slot:expr; $($rest:tt)*] ) => {
        $assembler.dup($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ADD; $($rest:tt)*] ) => {
        $assembler.add($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [EQ; $($rest:tt)*] ) => {{
        $assembler.eq($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [NEQ; $($rest:tt)*] ) => {{
        $assembler.neq($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [GT; $($rest:tt)*] ) => {{
        $assembler.gt($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [GTE; $($rest:tt)*] ) => {{
        $assembler.gte($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [LT; $($rest:tt)*] ) => {{
        $assembler.lt($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [LTE; $($rest:tt)*] ) => {{
        $assembler.lte($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [NOT; $($rest:tt)*] ) => {{
        $assembler.not($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    }};

    ($assembler:expr, $span:expr => [JUMP_IF_FALSE -> $loc:ident; $($rest:tt)*] ) => {
        $loc = $assembler.jump_if_false($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [JUMP_IF_TRUE -> $loc:ident; $($rest:tt)*] ) => {
        $loc = $assembler.jump_if_true($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };


    ($assembler:expr, $span:expr => [JUMP_BACK $offset:expr; $($rest:tt)*] ) => {
        $assembler.jump_back($offset, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_LOCAL $slot:expr; $($rest:tt)*] ) => {
        $assembler.load_local($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_LOCAL $slot:expr; $($rest:tt)*] ) => {
        $assembler.store_local($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSIGN_LOCAL $slot:expr; $($rest:tt)*] ) => {
        $assembler.assign_local($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_UPVALUE $slot:expr; $($rest:tt)*] ) => {
        $assembler.load_upvalue($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_UPVALUE $slot:expr; $($rest:tt)*] ) => {
        $assembler.store_upvalue($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSIGN_UPVALUE $slot:expr; $($rest:tt)*] ) => {
        $assembler.assign_upvalue($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_FIELD $field:expr; $($rest:tt)*] ) => {
        $assembler.load_field($field, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_FIELD $field:expr; $($rest:tt)*] ) => {
        $assembler.store_field($field, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSIGN_FIELD $field:expr; $($rest:tt)*] ) => {
        $assembler.assign_field($field, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_INDEX; $($rest:tt)*] ) => {
        $assembler.load_index($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_INDEX; $($rest:tt)*] ) => {
        $assembler.store_index($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSIGN_INDEX; $($rest:tt)*] ) => {
        $assembler.assign_index($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_METHOD $method:expr; $($rest:tt)*] ) => {
        $assembler.store_method($method, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [STORE_GLOBAL $global:expr; $($rest:tt)*] ) => {
        $assembler.store_global($global, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_FIELD_TO_LOCAL $field:expr, $slot:expr; $($rest:tt)*] ) => {
        $assembler.load_field_to_local($field, $slot, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [LOAD_MODULE $module:expr; $($rest:tt)*] ) => {
        $assembler.load_module($module, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [CALL $arg_count:expr; $($rest:tt)*] ) => {
        $assembler.call($arg_count, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [CLOSE_UPVALUES $variables:expr; $($rest:tt)*]) => {
        for variable in $variables {
            if variable.is_captured {
                $assembler.close_upvalue(variable.slot as u8, $span);
            }
        }

        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSERT_BOOL; $($rest:tt)*] ) => {
        $assembler.assert_bool($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [IS; $($rest:tt)*] ) => {
        $assembler.is($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSERT_TYPE_FOR_LOCAL $slot:expr; $($rest:tt)*] ) => {
        $assembler.assert_type_for_local($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSERT_TYPE_OR_NULL_FOR_LOCAL $slot:expr; $($rest:tt)*] ) => {
        $assembler.assert_type_or_null_for_local($slot, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSERT_TYPE_AND_RETURN; $($rest:tt)*] ) => {
        $assembler.assert_type_and_return($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [ASSERT_TYPE_OR_NULL_AND_RETURN; $($rest:tt)*] ) => {
        $assembler.assert_type_or_null_and_return($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [RET; $($rest:tt)*] ) => {
        $assembler.ret($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [PATCH_JUMP <- $into:expr; $($rest:tt)*] ) => {
        $assembler.patch_jump($into);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [{ $e:expr }; $($rest:tt)*] ) => {
        $e;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
}
