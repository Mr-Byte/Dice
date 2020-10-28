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
        self.data.put_u8(Instruction::PUSH_NULL.value());
    }

    pub fn push_unit(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_UNIT.value());
    }

    pub fn push_bool(&mut self, value: bool, span: Span) {
        let instruction = if value {
            Instruction::PUSH_TRUE
        } else {
            Instruction::PUSH_FALSE
        };

        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(instruction.value());
    }

    pub fn push_i0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_I0.value());
    }

    pub fn push_i1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_I1.value());
    }

    pub fn push_f0(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_F0.value());
    }

    pub fn push_f1(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_F1.value());
    }

    pub fn push_const(&mut self, value: Value, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::PUSH_CONST.value());

        self.source_map.insert(self.data.len() as u64, span);
        let const_pos = self.make_constant(value)?;
        self.data.put_u8(const_pos);

        Ok(())
    }

    pub fn closure(&mut self, value: Value, upvalues: &[UpvalueDescriptor], span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CREATE_CLOSURE.value());
        let fn_pos = self.make_constant(value)?;
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
        self.data.put_u8(Instruction::POP.value());
    }

    pub fn dup(&mut self, offset: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DUP.value());
        self.data.put_u8(offset);
    }

    pub fn create_list(&mut self, length: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CREATE_LIST.value());
        self.data.put_u8(length);
    }

    pub fn create_object(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CREATE_OBJECT.value());
    }

    pub fn create_class(&mut self, name: &str, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CREATE_CLASS.value());
        let name_slot = self.make_constant(Value::with_symbol(name))? as u8;
        self.data.put_u8(name_slot);

        Ok(())
    }

    pub fn mul(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::MUL.value());
    }

    pub fn div(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DIV.value());
    }

    pub fn rem(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::REM.value());
    }

    pub fn add(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::ADD.value());
    }

    pub fn sub(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::SUB.value());
    }

    pub fn eq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::EQ.value());
    }

    pub fn neq(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEQ.value());
    }

    pub fn is(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::IS.value());
    }

    pub fn gt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GT.value());
    }

    pub fn gte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::GTE.value());
    }

    pub fn lt(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LT.value());
    }

    pub fn lte(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LTE.value());
    }

    pub fn dice_roll(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DICE_ROLL.value());
    }

    pub fn range_inclusive(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RANGE_INCLUSIVE.value());
    }

    pub fn range_exclusive(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RANGE_EXCLUSIVE.value());
    }

    pub fn die_roll(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::DIE_ROLL.value());
    }

    pub fn neg(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NEG.value());
    }

    pub fn not(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::NOT.value());
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP.value());
        let patch_pos = self.data.len() as u64;
        self.data.put_i16(0);

        patch_pos
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump_if_false(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP_IF_FALSE.value());
        let patch_pos = self.data.len() as u64;
        self.data.put_i16(0);

        patch_pos
    }

    #[must_use = "Jumps must be patched."]
    pub fn jump_if_true(&mut self, span: Span) -> u64 {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::JUMP_IF_TRUE.value());
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
        self.data.put_u8(Instruction::JUMP.value());
        let offset = -((self.current_position() - position + 2) as i16);
        self.data.put_i16(offset);
    }

    pub fn current_position(&self) -> u64 {
        (self.data.len()) as u64
    }

    pub fn store_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::STORE_LOCAL.value());
        self.data.put_u8(slot);
    }

    pub fn load_local(&mut self, slot: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_LOCAL.value());
        self.data.put_u8(slot);
    }

    pub fn store_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::STORE_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn load_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn close_upvalue(&mut self, index: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CLOSE_UPVALUE.value());
        self.data.put_u8(index);
    }

    pub fn store_field(&mut self, field: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::STORE_FIELD.value());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn store_method(&mut self, method: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(method.into()))?;
        self.data.put_u8(Instruction::STORE_METHOD.value());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn load_field(&mut self, field: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::LOAD_FIELD.value());
        self.data.put_u8(const_slot);

        Ok(())
    }

    pub fn store_global(&mut self, global: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn store_global_impl(assembler: &mut Assembler, global: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(global))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::STORE_GLOBAL.value());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        store_global_impl(self, global.into(), span)
    }

    pub fn load_global(&mut self, global: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn load_global_impl(assembler: &mut Assembler, global: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(global))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::LOAD_GLOBAL.value());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        load_global_impl(self, global.into(), span)
    }

    pub fn load_module(&mut self, path: impl Into<Symbol>, span: Span) -> Result<(), CompilerError> {
        fn load_module_impl(assembler: &mut Assembler, path: Symbol, span: Span) -> Result<(), CompilerError> {
            let const_slot = assembler.make_constant(Value::with_symbol(path))?;

            assembler.source_map.insert(assembler.data.len() as u64, span);
            assembler.data.put_u8(Instruction::LOAD_MODULE.value());
            assembler.data.put_u8(const_slot);

            Ok(())
        }

        load_module_impl(self, path.into(), span)
    }

    pub fn load_index(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::LOAD_INDEX.value());
    }

    pub fn store_index(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::STORE_INDEX.value());
    }

    pub fn load_field_to_local(
        &mut self,
        field: impl Into<Symbol>,
        local_slot: u8,
        span: Span,
    ) -> Result<(), CompilerError> {
        self.source_map.insert(self.data.len() as u64, span);
        let const_slot = self.make_constant(Value::Symbol(field.into()))?;
        self.data.put_u8(Instruction::LOAD_FIELD_TO_LOCAL.value());
        self.data.put_u8(const_slot);
        self.data.put_u8(local_slot);

        Ok(())
    }

    pub fn call(&mut self, arg_count: u8, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::CALL.value());
        self.data.put_u8(arg_count);
    }

    pub fn ret(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::RETURN.value());
    }

    pub fn assert_bool(&mut self, span: Span) {
        self.source_map.insert(self.data.len() as u64, span);
        self.data.put_u8(Instruction::ASSERT_BOOL.value());
    }

    fn make_constant(&mut self, value: Value) -> Result<u8, CompilerError> {
        let position = if let Some(position) = self.constants.iter().position(|current| *current == value) {
            position
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };

        // NOTE: This could be alleviated by offering a long-form PUSH_CONST.
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
    ($assembler:expr, $span:expr => [PUSH_BOOL $value:expr; $($rest:tt)*] ) => {
        $assembler.push_bool($value, $span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
    ($assembler:expr, $span:expr => [PUSH_CONST $value:expr; $($rest:tt)*] ) => {
        $assembler.push_const($value, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
    ($assembler:expr, $span:expr => [CREATE_CLOSURE $value:expr, $upvalues:expr; $($rest:tt)*] ) => {
        $assembler.closure($value, $upvalues, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
    ($assembler:expr, $span:expr => [CREATE_CLASS $name:expr; $($rest:tt)*] ) => {
        $assembler.create_class($name, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [POP; $($rest:tt)*] ) => {
        $assembler.pop($span);
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

    ($assembler:expr, $span:expr => [LOAD_FIELD $field:expr; $($rest:tt)*] ) => {
        $assembler.load_field($field, $span)?;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
    ($assembler:expr, $span:expr => [STORE_FIELD $field:expr; $($rest:tt)*] ) => {
        $assembler.store_field($field, $span)?;
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

    ($assembler:expr, $span:expr => [RET; $($rest:tt)*] ) => {
        $assembler.ret($span);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

($assembler:expr, $span:expr => [PATCH_JUMP <- $value:expr; $($rest:tt)*] ) => {
        $assembler.patch_jump($value);
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };

    ($assembler:expr, $span:expr => [{ $e:expr }; $($rest:tt)*] ) => {
        $e;
        emit_bytecode! { $assembler, $span => [$($rest)*] }
    };
}
