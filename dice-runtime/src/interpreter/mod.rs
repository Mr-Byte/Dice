use std::collections::hash_map::Entry;

use dice_bytecode::{Bytecode, BytecodeCursor, Instruction};
use dice_core::{
    error::{
        codes::{
            CLASS_CANNOT_INHERIT_VALUE_TYPE, DIVIDE_BY_ZERO, GLOBAL_VARIABLE_ALREADY_DEFINED,
            GLOBAL_VARIABLE_UNDEFINED, TYPE_ASSERTION_BOOL_FAILURE, TYPE_ASSERTION_FAILURE,
            TYPE_ASSERTION_FUNCTION_FAILURE, TYPE_ASSERTION_NULLABILITY_FAILURE, TYPE_ASSERTION_NUMBER_FAILURE,
            TYPE_ASSERTION_SUPER_FAILURE,
        },
        context::{Context, ContextKind, INVALID_INDEX_TYPES, MISMATCHED_TYPE_ASSERTIONS},
        Error,
        ResultExt, trace::ErrorTrace,
    },
    protocol::operator::{ADD, DIV, EQ, GT, GTE, LT, LTE, MUL, NEQ, RANGE_EXCLUSIVE, RANGE_INCLUSIVE, REM, SUB},
    tags,
};

use crate::{module::ModuleLoader, runtime::Runtime, stack::StackFrame};
use crate::{
    upvalue::{Upvalue, UpvalueState},
    value::{Class, FnClosure, Object, Value, ValueKind},
};

mod helper;

impl<L> Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn execute(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: StackFrame,
        parent_upvalues: Option<&[Upvalue]>,
    ) -> Result<Value, Error> {
        let mut cursor = bytecode.cursor();

        // NOTE: Use IIFE to wrap the loop, to make building error traces easier.
        (|| {
            use Instruction::*;

            #[cfg(debug_assertions)]
            let initial_stack_depth = self.stack.len();

            while let Some(instruction) = cursor.read_instruction() {
                match instruction {
                    PushNull => self.stack.push(Value::Null),
                    PushUnit => self.stack.push(Value::Unit),
                    PushFalse => self.stack.push(Value::Bool(false)),
                    PushTrue => self.stack.push(Value::Bool(true)),
                    PushI0 => self.stack.push(Value::Int(0)),
                    PushI1 => self.stack.push(Value::Int(1)),
                    PushF0 => self.stack.push(Value::Float(0.0)),
                    PushF1 => self.stack.push(Value::Float(1.0)),
                    PushConst => self.push_const(bytecode, &mut cursor),
                    Pop => std::mem::drop(self.stack.pop()),
                    Swap => self.stack.swap(),
                    Dup => self.dup(&mut cursor),
                    CreateArray => self.create_list(&mut cursor),
                    CreateObject => self.create_object(),
                    InheritClass => self.inherit_class(&bytecode, &mut cursor)?,
                    CreateClosure => self.create_closure(bytecode, stack_frame, parent_upvalues, &mut cursor)?,
                    Negate => self.neg()?,
                    Not => self.not()?,
                    Multiply => self.mul()?,
                    Divide => self.div()?,
                    Remainder => self.rem()?,
                    Add => self.add()?,
                    Subtract => self.sub()?,
                    GreaterThan => self.gt()?,
                    GreaterThanOrEqual => self.gte()?,
                    LessThan => self.lt()?,
                    LessThanOrEqual => self.lte()?,
                    Equal => self.eq()?,
                    NotEqual => self.neq()?,
                    Is => self.is()?,
                    RangeExclusive => self.range_exclusive()?,
                    RangeInclusive => self.range_inclusive()?,
                    Jump => self.jump(&mut cursor)?,
                    JumpIfFalse => self.jump_if_false(&mut cursor)?,
                    JumpIfTrue => self.jump_if_true(&mut cursor)?,
                    LoadLocal => self.load_local(stack_frame, &mut cursor)?,
                    StoreLocal => self.store_local(stack_frame, &mut cursor)?,
                    AssignLocal => self.assign_local(stack_frame, &mut cursor)?,
                    LoadUpvalue => self.load_upvalue(parent_upvalues, &mut cursor)?,
                    StoreUpvalue => self.store_upvalue(parent_upvalues, &mut cursor)?,
                    AssignUpvalue => self.assign_upvalue(parent_upvalues, &mut cursor)?,
                    CloseUpvalue => self.close_upvalue(stack_frame, &mut cursor)?,
                    LoadGlobal => self.load_global(bytecode, &mut cursor)?,
                    StoreGlobal => self.store_global(bytecode, &mut cursor)?,
                    LoadField => self.load_field(bytecode, &mut cursor)?,
                    StoreField => self.store_field(bytecode, &mut cursor)?,
                    AssignField => self.assign_field(bytecode, &mut cursor)?,
                    LoadIndex => self.load_index()?,
                    StoreIndex => self.store_index()?,
                    AssignIndex => self.assign_index()?,
                    LoadMethod => self.load_method(bytecode, &mut cursor)?,
                    StoreMethod => self.store_method(bytecode, &mut cursor)?,
                    LoadFieldToLocal => self.load_field_to_local(bytecode, stack_frame, &mut cursor)?,
                    Call => self.call(&mut cursor)?,
                    CallSuper => self.call_super(&mut cursor)?,
                    LoadModule => self.load_module(&bytecode, &mut cursor)?,
                    AssertBool => self.assert_bool()?,
                    AssertTypeForLocal => self.assert_type_for_local(stack_frame, &mut cursor)?,
                    AssertTypeOrNullForLocal => self.assert_type_or_null_for_local(stack_frame, &mut cursor)?,
                    AssertTypeAndReturn => {
                        self.assert_type_and_return()?;
                        break;
                    }
                    AssertTypeOrNullAndReturn => {
                        self.assert_type_or_null_and_return()?;
                        break;
                    }
                    Return => break,
                };
            }

            // NOTE: subtract 1 to compensate for the last item of the stack not yet being popped.
            #[cfg(debug_assertions)]
            assert_eq!(
                initial_stack_depth,
                self.stack.len() - 1,
                "Stack was left in a bad state. Initial depth {}, final depth {}",
                initial_stack_depth,
                self.stack.len() - 1
            );

            Ok(self.stack.pop())
        })()
        .push_trace(|| ErrorTrace::from_bytecode(&bytecode, cursor.last_instruction_offset()))
    }

    fn jump(&mut self, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let offset = cursor.read_offset();
        cursor.offset_position(offset);

        Ok(())
    }

    fn dup(&mut self, cursor: &mut BytecodeCursor) {
        let value = self.stack.peek_mut(cursor.read_u8() as usize).clone();
        self.stack.push(value);
    }

    fn assert_bool(&mut self) -> Result<(), Error> {
        if self.stack.peek_mut(0).kind() != ValueKind::Bool {
            return Err(Error::new(TYPE_ASSERTION_BOOL_FAILURE));
        }

        Ok(())
    }

    fn assert_type_for_local(&mut self, stack_frame: StackFrame, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let class = self.stack.pop();
        let class = class.as_class()?;
        let value = &self.stack[stack_frame][cursor.read_u8() as usize];

        if *value == Value::Null {
            return Err(Error::new(TYPE_ASSERTION_NULLABILITY_FAILURE));
        }

        self.assert_type(&class, &value)
    }

    fn assert_type_or_null_for_local(
        &mut self,
        stack_frame: StackFrame,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), Error> {
        let class = self.stack.pop();
        let class = class.as_class()?;
        let value = &self.stack[stack_frame][cursor.read_u8() as usize];

        if *value == Value::Null {
            return Ok(());
        }

        self.assert_type(&class, &value)
    }

    fn assert_type_and_return(&mut self) -> Result<(), Error> {
        let class = self.stack.pop();
        let class = class.as_class()?;
        let value = self.stack.peek(0);

        if *value == Value::Null {
            return Err(Error::new(TYPE_ASSERTION_NULLABILITY_FAILURE));
        }

        self.assert_type(&class, &value)
    }

    fn assert_type_or_null_and_return(&mut self) -> Result<(), Error> {
        let class = self.stack.pop();
        let class = class.as_class()?;
        let value = self.stack.peek(0);

        if *value == Value::Null {
            return Ok(());
        }

        self.assert_type(&class, &value)
    }

    fn assert_type(&self, class: &Class, value: &&Value) -> Result<(), Error> {
        let actual_class = value
            .as_object()
            .ok()
            .and_then(|object| object.class())
            .or_else(|| self.value_class_mapping.get(&value.kind()).cloned());
        let is_type = actual_class
            .as_ref()
            .map_or(false, |local_class| local_class.is_class(&class));

        if is_type {
            Ok(())
        } else {
            let expected_type = class.name().as_string();
            let actual_type =
                actual_class.map_or(String::from("<unknown>"), |local_class| local_class.name().as_string());

            Err(Error::new(TYPE_ASSERTION_FAILURE).push_context(
                Context::new(MISMATCHED_TYPE_ASSERTIONS, ContextKind::Note).with_tags(tags! {
                    expected => expected_type,
                    actual => actual_type
                }),
            ))
        }
    }

    fn not(&mut self) -> Result<(), Error> {
        match self.stack.peek_mut(0) {
            Value::Bool(value) => *value = !*value,
            _ => return Err(Error::new(TYPE_ASSERTION_BOOL_FAILURE)),
        }

        Ok(())
    }

    fn neg(&mut self) -> Result<(), Error> {
        match self.stack.peek_mut(0) {
            Value::Int(value) => *value = -*value,
            Value::Float(value) => *value = -*value,
            _ => {
                return Err(Error::new(TYPE_ASSERTION_NUMBER_FAILURE));
            }
        }

        Ok(())
    }

    fn mul(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs *= rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs *= rhs,
            (rhs, _) => self.call_binary_op(&MUL, rhs)?,
        }

        Ok(())
    }

    fn div(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => {
                if rhs == 0 {
                    return Err(Error::new(DIVIDE_BY_ZERO));
                }

                *lhs /= rhs;
            }
            (Value::Float(rhs), Value::Float(lhs)) => *lhs /= rhs,
            (rhs, _) => self.call_binary_op(&DIV, rhs)?,
        }

        Ok(())
    }

    fn rem(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => {
                if rhs == 0 {
                    return Err(Error::new(DIVIDE_BY_ZERO));
                }

                *lhs %= rhs;
            }
            (Value::Float(rhs), Value::Float(lhs)) => *lhs %= rhs,
            (rhs, _) => self.call_binary_op(&REM, rhs)?,
        }

        Ok(())
    }

    fn add(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs += rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs += rhs,
            (rhs, _) => self.call_binary_op(&ADD, rhs)?,
        }

        Ok(())
    }

    fn gt(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs &= !rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs > rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs > rhs),
            (rhs, _) => self.call_binary_op(&GT, rhs)?,
        }

        Ok(())
    }

    fn gte(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs >= rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs >= rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs >= rhs),
            (rhs, _) => self.call_binary_op(&GTE, rhs)?,
        }

        Ok(())
    }

    fn lt(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = !(*lhs) & rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs < rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs < rhs),
            (rhs, _) => self.call_binary_op(&LT, rhs)?,
        }

        Ok(())
    }

    fn lte(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs <= rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs <= rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs <= rhs),
            (rhs, _) => self.call_binary_op(&LTE, rhs)?,
        }

        Ok(())
    }

    fn sub(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs -= rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs -= rhs,
            (rhs, _) => self.call_binary_op(&SUB, rhs)?,
        }

        Ok(())
    }

    fn eq(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Null, Value::Null) => *self.stack.peek_mut(0) = Value::Bool(true),
            (Value::Null, _) => *self.stack.peek_mut(0) = Value::Bool(false),
            (_, Value::Null) => *self.stack.peek_mut(0) = Value::Bool(false),
            (Value::Unit, Value::Unit) => *self.stack.peek_mut(0) = Value::Bool(true),
            (Value::Unit, _) => *self.stack.peek_mut(0) = Value::Bool(false),
            (_, Value::Unit) => *self.stack.peek_mut(0) = Value::Bool(false),
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs == rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs == rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs == rhs),
            (rhs, _) => self.call_binary_op(&EQ, rhs)?,
        }

        Ok(())
    }

    fn neq(&mut self) -> Result<(), Error> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Null, Value::Null) => *self.stack.peek_mut(0) = Value::Bool(false),
            (Value::Null, _) => *self.stack.peek_mut(0) = Value::Bool(true),
            (_, Value::Null) => *self.stack.peek_mut(0) = Value::Bool(true),
            (Value::Unit, Value::Unit) => *self.stack.peek_mut(0) = Value::Bool(false),
            (Value::Unit, _) => *self.stack.peek_mut(0) = Value::Bool(true),
            (_, Value::Unit) => *self.stack.peek_mut(0) = Value::Bool(true),
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs != rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs != rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs != rhs),
            (rhs, _) => self.call_binary_op(&NEQ, rhs)?,
        }

        Ok(())
    }

    fn range_inclusive(&mut self) -> Result<(), Error> {
        let rhs = self.stack.pop();
        self.call_binary_op(&RANGE_INCLUSIVE, rhs)
    }

    fn range_exclusive(&mut self) -> Result<(), Error> {
        let rhs = self.stack.pop();
        self.call_binary_op(&RANGE_EXCLUSIVE, rhs)
    }

    fn is(&mut self) -> Result<(), Error> {
        let class = self.stack.pop();
        let class = class.as_class()?;
        let instance = self.stack.peek(0);
        let is_type = self.is_value_of_type(&instance, &class)?;

        *self.stack.peek_mut(0) = Value::Bool(is_type);

        Ok(())
    }

    fn create_list(&mut self, cursor: &mut BytecodeCursor) {
        let count = cursor.read_u8() as usize;
        let items = self.stack.pop_count(count);

        self.stack.push(Value::Array(items.to_vec().into()));
    }

    fn create_object(&mut self) {
        let object = Object::new(self.any_class.clone());

        self.stack.push(Value::Object(object));
    }

    fn inherit_class(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let name_slot = cursor.read_u8() as usize;
        let name = bytecode.constants()[name_slot].as_symbol()?;
        let base = self.stack.pop().as_class()?;

        if base != self.any_class
            && self
                .value_class_mapping
                .iter()
                .find(|(_, value_class)| base == **value_class)
                .is_some()
        {
            return Err(Error::new(CLASS_CANNOT_INHERIT_VALUE_TYPE));
        }

        let class = Class::with_base(name, base);

        self.stack.push(Value::Class(class));

        Ok(())
    }

    fn push_const(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) {
        let const_pos = cursor.read_u8() as usize;
        let value = bytecode.constants()[const_pos].clone();
        self.stack.push(value);
    }

    fn jump_if_false(&mut self, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let offset = cursor.read_offset();
        let value = self.stack.pop().as_bool()?;

        if !value {
            cursor.offset_position(offset)
        }

        Ok(())
    }

    fn jump_if_true(&mut self, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let offset = cursor.read_offset();
        let value = self.stack.pop().as_bool()?;

        if value {
            cursor.offset_position(offset)
        }

        Ok(())
    }

    fn load_local(&mut self, stack_frame: StackFrame, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let slot = cursor.read_u8() as usize;
        let frame = &self.stack[stack_frame];
        let value = frame[slot].clone();
        self.stack.push(value);

        Ok(())
    }

    fn store_local(&mut self, stack_frame: StackFrame, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let value = self.stack.pop();
        let slot = cursor.read_u8() as usize;

        self.stack[stack_frame][slot] = value.clone();
        self.stack.push(value);

        Ok(())
    }

    fn assign_local(&mut self, stack_frame: StackFrame, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let value = self.stack.pop();
        let slot = cursor.read_u8() as usize;

        self.stack[stack_frame][slot] = value;
        self.stack.push(Value::Unit);

        Ok(())
    }

    fn load_upvalue(&mut self, parent_upvalues: Option<&[Upvalue]>, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        if let Some(parent_upvalues) = parent_upvalues {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = parent_upvalues[upvalue_slot].clone();
            let value = match &*upvalue.state_mut() {
                UpvalueState::Open(slot) => self.stack[*slot].clone(),
                UpvalueState::Closed(value) => value.clone(),
            };

            self.stack.push(value);

            Ok(())
        } else {
            unreachable!("LoadUpvalue used in non-closure context.")
        }
    }

    fn store_upvalue(&mut self, parent_upvalues: Option<&[Upvalue]>, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        if let Some(parent_upvalues) = parent_upvalues {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = parent_upvalues[upvalue_slot].clone();
            let value = self.stack.pop();
            let result = match &mut *upvalue.state_mut() {
                UpvalueState::Open(slot) => {
                    self.stack[*slot] = value.clone();
                    value
                }
                UpvalueState::Closed(closed_value) => {
                    *closed_value = value.clone();
                    value
                }
            };

            self.stack.push(result);

            Ok(())
        } else {
            unreachable!("StoreUpvalue used in non-closure context.")
        }
    }

    fn assign_upvalue(
        &mut self,
        parent_upvalues: Option<&[Upvalue]>,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), Error> {
        if let Some(parent_upvalues) = parent_upvalues {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = parent_upvalues[upvalue_slot].clone();
            let value = self.stack.pop();
            match &mut *upvalue.state_mut() {
                UpvalueState::Open(slot) => self.stack[*slot] = value,
                UpvalueState::Closed(closed_value) => *closed_value = value,
            };

            self.stack.push(Value::Unit);

            Ok(())
        } else {
            unreachable!("AssignUpvalue used in non-closure context.")
        }
    }

    fn close_upvalue(&mut self, stack_frame: StackFrame, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let offset = cursor.read_u8() as usize;
        let value = std::mem::replace(&mut self.stack[stack_frame][offset], Value::Null);
        let offset = stack_frame.start() + offset;
        let found_upvalue = self.find_open_upvalue(offset);

        if let Some((index, _)) = found_upvalue {
            if let Some(upvalue) = self.open_upvalues.remove(index) {
                upvalue.close(value);
            }
        }

        Ok(())
    }

    fn store_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let const_pos = cursor.read_u8() as usize;
        let value = &bytecode.constants()[const_pos];
        let global_name = value.as_symbol()?;
        let global = self.stack.pop();

        match self.globals.entry(global_name.clone()) {
            Entry::Occupied(_) => {
                return Err(Error::new(GLOBAL_VARIABLE_ALREADY_DEFINED).with_tags(tags! {
                    name => global_name.to_string()
                }))
            }
            Entry::Vacant(entry) => {
                entry.insert(global);
            }
        }

        Ok(())
    }

    fn load_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let const_pos = cursor.read_u8() as usize;
        let global = bytecode.constants()[const_pos].as_symbol()?;
        let value = self.globals.get(&global).cloned().ok_or_else(|| {
            Error::new(GLOBAL_VARIABLE_UNDEFINED).with_tags(tags! {
                name => global.to_string()
            })
        })?;

        self.stack.push(value);

        Ok(())
    }

    fn load_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;

        let value = self.stack.pop();
        let value = self.get_field(key, value)?;

        self.stack.push(value);

        Ok(())
    }

    fn store_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;
        let value = self.stack.pop();
        let object = self.stack.pop();
        let object = object.as_object()?;

        object.set_field(key, value.clone());
        self.stack.push(value);

        Ok(())
    }

    fn assign_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;
        let value = self.stack.pop();
        let object = self.stack.pop();
        let object = object.as_object()?;

        object.set_field(key, value);
        self.stack.push(Value::Unit);

        Ok(())
    }

    fn load_index(&mut self) -> Result<(), Error> {
        let index = self.stack.pop();
        let target = self.stack.peek(0);
        let result = match target {
            Value::Array(array) if index.kind() == ValueKind::Int => {
                let index = index.as_int()?;
                array.elements().get(index as usize).cloned().unwrap_or(Value::Null)
            }
            target => {
                let field = index
                    .as_symbol()
                    .push_context(|| Context::new(INVALID_INDEX_TYPES, ContextKind::Note))?;
                self.get_field(field, target.clone())?
            }
        };

        *self.stack.peek_mut(0) = result;

        Ok(())
    }

    fn store_index(&mut self) -> Result<(), Error> {
        let value = self.stack.pop();
        let index = self.stack.pop();
        let target = self.stack.peek_mut(0);

        match target {
            Value::Array(array) if index.kind() == ValueKind::Int => {
                let index = index.as_int()?;
                array.elements_mut()[index as usize] = value.clone();
                *target = value;
            }
            target => {
                let object = target.as_object()?;
                let field = index
                    .as_symbol()
                    .push_context(|| Context::new(INVALID_INDEX_TYPES, ContextKind::Note))?;
                object.set_field(field, value.clone());
                *target = value;
            }
        };

        Ok(())
    }

    fn assign_index(&mut self) -> Result<(), Error> {
        let value = self.stack.pop();
        let index = self.stack.pop();
        let target = self.stack.peek_mut(0);

        match target {
            Value::Array(array) if index.kind() == ValueKind::Int => {
                let index = index.as_int()?;
                array.elements_mut()[index as usize] = value;
                *target = Value::Unit;
            }
            target => {
                let object = target.as_object()?;
                let field = index
                    .as_symbol()
                    .push_context(|| Context::new(INVALID_INDEX_TYPES, ContextKind::Note))?;
                object.set_field(field, value);
                *target = Value::Unit;
            }
        };

        Ok(())
    }

    fn load_method(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;
        let receiver = self.stack.pop();
        let class = self.stack.pop().as_class()?;

        if !self.is_value_of_type(&receiver, &class)? {
            return Err(Error::new(TYPE_ASSERTION_SUPER_FAILURE));
        }

        let method = self.get_method(Some(&class), key, &receiver);
        self.stack.push(method);

        Ok(())
    }

    fn store_method(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;
        let value = self.stack.pop();
        let object = self.stack.pop();
        let class = object.as_class()?;

        class.set_method(key, value);

        Ok(())
    }

    fn load_field_to_local(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: StackFrame,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), Error> {
        let key_index = cursor.read_u8() as usize;
        let local_slot = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_symbol()?;
        let value = self.stack.pop();
        let value = self.get_field(key, value)?;

        self.stack[stack_frame][local_slot] = value.clone();
        self.stack.push(value);

        Ok(())
    }

    fn create_closure(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: StackFrame,
        parent_upvalues: Option<&[Upvalue]>,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), Error> {
        let const_pos = cursor.read_u8() as usize;

        match bytecode.constants()[const_pos] {
            Value::FnScript(ref fn_script) => {
                let upvalue_count = fn_script.bytecode().upvalue_count();
                let mut upvalues = Vec::with_capacity(upvalue_count);

                for _ in 0..upvalue_count {
                    let is_parent_local = cursor.read_u8() == 1;
                    let index = cursor.read_u8() as usize;

                    if is_parent_local {
                        let offset = stack_frame.start() + index;
                        match self.find_open_upvalue(offset) {
                            None => {
                                let upvalue = Upvalue::new_open(stack_frame.start() + index);
                                self.open_upvalues.push_back(upvalue.clone());
                                upvalues.push(upvalue);
                            }
                            Some((_, upvalue)) => upvalues.push(upvalue),
                        };
                    } else if let Some(parent_upvalues) = parent_upvalues {
                        let upvalue = parent_upvalues[index].clone();
                        upvalues.push(upvalue);
                    } else {
                        // NOTE: Produce an unreachable here. This case should never execute, but this is a sanity check to ensure it doesn't.
                        unreachable!("No parent scope found.")
                    }
                }

                let closure = Value::FnClosure(FnClosure::new(fn_script.clone(), upvalues.into_boxed_slice()));
                self.stack.push(closure);
            }
            _ => return Err(Error::new(TYPE_ASSERTION_FUNCTION_FAILURE)),
        }

        Ok(())
    }

    pub fn call(&mut self, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let arg_count = cursor.read_u8() as usize;
        self.call_fn(arg_count)
    }

    pub fn call_super(&mut self, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let arg_count = cursor.read_u8() as usize;
        let super_ = self.stack.pop().as_class()?;
        let receiver = self.stack.peek(arg_count).clone();
        let result = self.call_class_constructor(arg_count, &super_, receiver)?;

        self.stack.push(result);

        Ok(())
    }

    fn load_module(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), Error> {
        let module_slot = cursor.read_u8() as usize;
        let module_name = bytecode.constants()[module_slot].as_symbol()?;
        let module = match self.loaded_modules.entry(module_name.clone()) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let export = Value::Object(Object::new(self.module_class.clone()));
                entry.insert(export.clone());

                let module = self.module_loader.load_module(module_name)?;
                self.run_module(module.bytecode, export)?
            }
        };

        self.stack.push(module);

        Ok(())
    }
}
