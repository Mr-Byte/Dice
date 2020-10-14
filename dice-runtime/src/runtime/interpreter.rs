use crate::{
    module::ModuleLoader,
    runtime::{call_frame::CallFrame, Runtime},
};
use dice_core::{
    bytecode::{instruction::Instruction, Bytecode, BytecodeCursor},
    constants::{ADD, DIV, GT, GTE, LT, LTE, MUL, REM, SUB},
    id::type_id::TypeId,
    upvalue::{Upvalue, UpvalueState},
    value::{Class, FnBound, FnClosure, Object, Value},
};
use dice_error::runtime_error::RuntimeError;
use std::collections::hash_map::Entry;

impl<L> Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn execute_bytecode(
        &mut self,
        bytecode: &Bytecode,
        call_frame: CallFrame,
        closure: Option<FnClosure>,
    ) -> Result<Value, RuntimeError> {
        let initial_stack_depth = self.stack.len();
        let mut cursor = bytecode.cursor();

        while let Some(instruction) = cursor.read_instruction() {
            match instruction {
                Instruction::PUSH_NULL => self.stack.push(Value::Null),
                Instruction::PUSH_UNIT => self.stack.push(Value::Unit),
                Instruction::PUSH_FALSE => self.stack.push(Value::Bool(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::Bool(true)),
                Instruction::PUSH_I0 => self.stack.push(Value::Int(0)),
                Instruction::PUSH_I1 => self.stack.push(Value::Int(1)),
                Instruction::PUSH_F0 => self.stack.push(Value::Float(0.0)),
                Instruction::PUSH_F1 => self.stack.push(Value::Float(1.0)),
                Instruction::PUSH_CONST => self.push_const(bytecode, &mut cursor),
                Instruction::POP => std::mem::drop(self.stack.pop()),
                Instruction::DUP => self.dup(&mut cursor),
                Instruction::CREATE_LIST => self.create_list(&mut cursor),
                Instruction::CREATE_OBJECT => self.create_object(),
                Instruction::CREATE_CLASS => self.create_class(&bytecode, &mut cursor)?,
                Instruction::CREATE_CLOSURE => self.create_closure(bytecode, call_frame, &closure, &mut cursor)?,
                Instruction::NEG => self.neg()?,
                Instruction::NOT => self.not()?,
                Instruction::MUL => self.mul()?,
                Instruction::DIV => self.div()?,
                Instruction::REM => self.rem()?,
                Instruction::ADD => self.add()?,
                Instruction::SUB => self.sub()?,
                Instruction::GT => self.gt()?,
                Instruction::GTE => self.gte()?,
                Instruction::LT => self.lt()?,
                Instruction::LTE => self.lte()?,
                Instruction::EQ => self.eq(),
                Instruction::NEQ => self.neq(),
                Instruction::JUMP => self.jump(&mut cursor),
                Instruction::JUMP_IF_FALSE => self.jump_if_false(&mut cursor)?,
                Instruction::LOAD_LOCAL => self.load_local(call_frame, &mut cursor),
                Instruction::STORE_LOCAL => self.store_local(call_frame, &mut cursor),
                Instruction::LOAD_UPVALUE => self.load_upvalue(&closure, &mut cursor),
                Instruction::STORE_UPVALUE => self.store_upvalue(&closure, &mut cursor),
                Instruction::CLOSE_UPVALUE => self.close_upvalue(call_frame, &mut cursor),
                Instruction::LOAD_GLOBAL => self.load_global(bytecode, &mut cursor)?,
                Instruction::STORE_GLOBAL => self.store_global(bytecode, &mut cursor)?,
                Instruction::LOAD_FIELD => self.load_field(bytecode, &mut cursor)?,
                Instruction::STORE_FIELD => self.store_field(bytecode, &mut cursor)?,
                Instruction::LOAD_INDEX => self.load_index()?,
                Instruction::STORE_INDEX => self.store_index()?,
                Instruction::STORE_METHOD => self.store_method(bytecode, &mut cursor)?,
                Instruction::CALL => self.call(&mut cursor)?,
                Instruction::ASSERT_BOOL => self.assert_bool()?,
                Instruction::LOAD_MODULE => self.load_module(&bytecode, &mut cursor)?,
                Instruction::RETURN => break,
                unknown => return Err(RuntimeError::UnknownInstruction(unknown.value())),
            }
        }

        // NOTE: subtract 1 to compensate for the last item of the stack not yet being popped.
        let final_stack_depth = self.stack.len() - 1;

        assert_eq!(
            initial_stack_depth, final_stack_depth,
            "Stack was left in a bad state. Initial depth {}, final depth {}",
            initial_stack_depth, final_stack_depth
        );

        Ok(self.stack.pop())
    }

    #[inline]
    fn jump(&mut self, cursor: &mut BytecodeCursor) {
        let offset = cursor.read_offset();
        cursor.offset_position(offset);
    }

    #[inline]
    fn dup(&mut self, cursor: &mut BytecodeCursor) {
        let value = self.stack.peek_mut(cursor.read_u8() as usize).clone();
        self.stack.push(value);
    }

    #[inline]
    fn assert_bool(&mut self) -> Result<(), RuntimeError> {
        if !self.stack.peek_mut(0).is_bool() {
            return Err(RuntimeError::Aborted(String::from(
                "Right hand side must evaluate to a boolean.",
            )));
        }

        Ok(())
    }

    #[inline]
    fn not(&mut self) -> Result<(), RuntimeError> {
        match self.stack.peek_mut(0) {
            Value::Bool(value) => *value = !*value,
            _ => return Err(RuntimeError::Aborted(String::from("Value must be a boolean."))),
        }

        Ok(())
    }

    #[inline]
    fn neg(&mut self) -> Result<(), RuntimeError> {
        match self.stack.peek_mut(0) {
            Value::Int(value) => *value = -*value,
            Value::Float(value) => *value = -*value,
            _ => {
                return Err(RuntimeError::Aborted(String::from(
                    "Can only negate an integer or float.",
                )))
            }
        }

        Ok(())
    }

    #[inline]
    fn mul(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs *= rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs *= rhs,
            (rhs, _) => self.call_bin_op(MUL, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn div(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => {
                if rhs == 0 {
                    return Err(RuntimeError::DivideByZero);
                }

                *lhs /= rhs;
            }
            (Value::Float(rhs), Value::Float(lhs)) => *lhs /= rhs,
            (rhs, _) => self.call_bin_op(DIV, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn rem(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => {
                if rhs == 0 {
                    return Err(RuntimeError::DivideByZero);
                }

                *lhs %= rhs;
            }
            (Value::Float(rhs), Value::Float(lhs)) => *lhs %= rhs,
            (rhs, _) => self.call_bin_op(REM, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn add(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs += rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs += rhs,
            (rhs, _) => self.call_bin_op(ADD, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn gt(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs &= !rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs > rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs > rhs),
            (rhs, _) => self.call_bin_op(GT, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn gte(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs >= rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs >= rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs >= rhs),
            (rhs, _) => self.call_bin_op(GTE, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn lt(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = !(*lhs) & rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs < rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs < rhs),
            (rhs, _) => self.call_bin_op(LT, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn lte(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Bool(rhs), Value::Bool(lhs)) => *lhs = *lhs <= rhs,
            (Value::Int(rhs), Value::Int(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs <= rhs),
            (Value::Float(rhs), Value::Float(lhs)) => *self.stack.peek_mut(0) = Value::Bool(*lhs <= rhs),
            (rhs, _) => self.call_bin_op(LTE, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn sub(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek_mut(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs -= rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs -= rhs,
            (rhs, _) => self.call_bin_op(SUB, rhs)?,
        }

        Ok(())
    }

    #[inline]
    fn eq(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.peek_mut(0);

        *lhs = Value::Bool(rhs == *lhs);
    }

    #[inline]
    fn neq(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.peek_mut(0);

        *lhs = Value::Bool(rhs != *lhs);
    }

    #[inline]
    fn call_bin_op(&mut self, operator: &str, rhs: Value) -> Result<(), RuntimeError> {
        // TODO: Resolve operators from class members.
        let value = self
            .globals
            .get(operator)
            .ok_or_else(|| RuntimeError::Aborted("No global operator defined.".to_owned()))?;
        let lhs = self.stack.pop();

        self.stack.push(value.clone());
        self.stack.push(lhs);
        self.stack.push(rhs);
        self.call_fn(2)?;

        Ok(())
    }

    fn create_list(&mut self, cursor: &mut BytecodeCursor) {
        let count = cursor.read_u8() as usize;
        let items = self.stack.pop_count(count);

        self.stack.push(Value::List(items.to_vec().into()));
    }

    fn create_object(&mut self) {
        let object = Object::new(TypeId::new(None, None, "Object"), None);

        self.stack.push(Value::Object(object));
    }

    fn create_class(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let name_slot = cursor.read_u8() as usize;
        let path_slot = cursor.read_u8() as usize;
        let name = bytecode.constants()[name_slot].as_str()?;
        let path = bytecode.constants()[path_slot].as_str()?;
        let class = Class::new(name.to_owned(), path.to_owned());

        self.stack.push(Value::Class(class));

        Ok(())
    }

    fn push_const(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) {
        let const_pos = cursor.read_u8() as usize;
        let value = bytecode.constants()[const_pos].clone();
        self.stack.push(value);
    }

    fn jump_if_false(&mut self, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let offset = cursor.read_offset();
        let value = self.stack.pop().as_bool()?;

        if !value {
            cursor.offset_position(offset)
        }

        Ok(())
    }

    fn load_local(&mut self, call_frame: CallFrame, cursor: &mut BytecodeCursor) {
        // TODO Bounds check the slot?
        let slot = cursor.read_u8() as usize;
        let frame = &self.stack[call_frame];
        let value = frame[slot].clone();
        self.stack.push(value);
    }

    fn store_local(&mut self, call_frame: CallFrame, cursor: &mut BytecodeCursor) {
        let value = self.stack.pop();
        let slot = cursor.read_u8() as usize;

        self.stack[call_frame][slot] = value.clone();
        self.stack.push(value);
    }

    fn load_upvalue(&mut self, closure: &Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = closure.upvalues[upvalue_slot].clone();
            let value = match &*upvalue.state_mut() {
                UpvalueState::Open(slot) => self.stack.slot(*slot).clone(),
                UpvalueState::Closed(value) => value.clone(),
            };

            self.stack.push(value);
        } else {
            unreachable!("LOAD_UPVALUE used in non-closure context.")
        }
    }

    fn store_upvalue(&mut self, closure: &Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure {
            let upvalue_slot = cursor.read_u8() as usize;
            let upvalue = closure.upvalues[upvalue_slot].clone();
            let value = self.stack.pop();
            let result = match &mut *upvalue.state_mut() {
                UpvalueState::Open(slot) => {
                    *self.stack.slot(*slot) = value.clone();
                    value
                }
                UpvalueState::Closed(closed_value) => {
                    *closed_value = value.clone();
                    value
                }
            };

            self.stack.push(result)
        } else {
            unreachable!("STORE_UPVALUE used in non-closure context.")
        }
    }

    fn close_upvalue(&mut self, call_frame: CallFrame, cursor: &mut BytecodeCursor) {
        let offset = cursor.read_u8() as usize;
        let value = std::mem::replace(&mut self.stack[call_frame][offset], Value::Null);
        let offset = call_frame.start() + offset;
        let found_upvalue = self.find_open_upvalue(offset);

        if let Some((index, _)) = found_upvalue {
            if let Some(upvalue) = self.open_upvalues.remove(index) {
                upvalue.close(value);
            }
        }
    }

    fn store_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;
        let value = &bytecode.constants()[const_pos];
        let global_name = value.as_str()?.to_owned();
        let global = self.stack.pop();

        match self.globals.entry(global_name) {
            Entry::Occupied(_) => todo!("Return error that global already exists."),
            Entry::Vacant(entry) => {
                entry.insert(global);
            }
        }

        Ok(())
    }

    fn load_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;
        let global = bytecode.constants()[const_pos].as_str()?;
        let value = self
            .globals
            .get(global)
            .cloned()
            .ok_or_else(|| RuntimeError::VariableNotFound(global.to_owned()))?;

        self.stack.push(value);

        Ok(())
    }

    fn store_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_str()?;
        let value = self.stack.pop();
        let object = self.stack.pop();
        let object = object.as_object()?;

        object.fields_mut().insert(key.to_owned(), value.clone());
        self.stack.push(value);

        Ok(())
    }

    fn load_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_str()?;
        let value = self.stack.pop();
        let object = value.as_object()?;
        let fields = object.fields();
        let value = match fields.get(key) {
            Some(field) => field.clone(),
            None => match object.class() {
                Some(class) => match class.methods().get(key) {
                    Some(method) => Value::FnBound(FnBound::new(value.clone(), method.clone())),
                    None => Value::Null,
                },
                None => Value::Null,
            },
        };

        self.stack.push(value);

        Ok(())
    }

    // TODO: Work on this to make classes index-able.
    #[inline]
    fn store_index(&mut self) -> Result<(), RuntimeError> {
        let value = self.stack.pop();
        let index = self.stack.pop();
        let target = self.stack.peek_mut(0);

        match target {
            Value::Object(object) => {
                let field = index.as_str()?;
                object.fields_mut().insert(field.to_owned(), value.clone());
                *target = value;
            }
            Value::List(list) => {
                let index = index.as_int()?;
                list.elements_mut()[index as usize] = value.clone();
                *target = value;
            }
            _ => todo!("Return invalid index target error."),
        };

        Ok(())
    }

    #[inline]
    fn load_index(&mut self) -> Result<(), RuntimeError> {
        let index = self.stack.pop();
        let target = self.stack.peek_mut(0);

        let result = match target {
            Value::Object(object) => {
                let field = index.as_str()?;
                object.fields().get(field).cloned()
            }
            Value::List(list) => {
                let index = index.as_int()?;
                list.elements().get(index as usize).cloned()
            }
            _ => todo!("Return invalid index target error."),
        };

        *target = result.unwrap_or(Value::Null);

        Ok(())
    }

    fn store_method(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let key_index = cursor.read_u8() as usize;
        let key = bytecode.constants()[key_index].as_str()?;
        let value = self.stack.pop();
        let object = self.stack.pop();
        let class = object.as_class()?;

        class.methods_mut().insert(key.to_owned(), value);

        Ok(())
    }

    fn create_closure(
        &mut self,
        bytecode: &Bytecode,
        call_frame: CallFrame,
        closure: &Option<FnClosure>,
        cursor: &mut BytecodeCursor,
    ) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;

        match bytecode.constants()[const_pos] {
            Value::FnScript(ref fn_script) => {
                let upvalue_count = fn_script.bytecode.upvalue_count();
                let mut upvalues = Vec::with_capacity(upvalue_count);

                for _ in 0..upvalue_count {
                    let is_parent_local = cursor.read_u8() == 1;
                    let index = cursor.read_u8() as usize;

                    if is_parent_local {
                        let offset = call_frame.start() + index;
                        match self.find_open_upvalue(offset) {
                            None => {
                                let upvalue = Upvalue::new_open(call_frame.start() + index);
                                self.open_upvalues.push_back(upvalue.clone());
                                upvalues.push(upvalue);
                            }
                            Some((_, upvalue)) => {
                                upvalues.push(upvalue);
                            }
                        };
                    } else if let Some(closure) = closure {
                        let upvalue = closure.upvalues[index].clone();
                        upvalues.push(upvalue);
                    } else {
                        // NOTE: Produce an unreachable here. This case should never execute, but this is a sanity check to ensure it doesn't.
                        unreachable!("No parent scope found.")
                    }
                }

                let closure = Value::FnClosure(FnClosure::new(fn_script.clone(), upvalues.into_boxed_slice()));
                self.stack.push(closure);
            }
            _ => return Err(RuntimeError::NotAFunction),
        }

        Ok(())
    }

    #[inline]
    fn call(&mut self, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let arg_count = cursor.read_u8() as usize;
        self.call_fn(arg_count)
    }

    // TODO: Replace this mutually recursive call with an execution stack to prevent the thread's stack from overflowing.
    pub(super) fn call_fn(&mut self, arg_count: usize) -> Result<(), RuntimeError> {
        let (target, arg_count, receiver) = match self.stack.peek(arg_count) {
            Value::FnBound(fn_bound) => (fn_bound.function.clone(), arg_count, Some(fn_bound.receiver.clone())),
            value => (value.clone(), arg_count, None),
        };

        let (bytecode, closure) = match &target {
            Value::FnClosure(closure) => {
                let fn_script = &closure.fn_script;

                if arg_count != fn_script.arity {
                    return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
                }

                (fn_script.bytecode.clone(), Some(closure.clone()))
            }
            Value::FnScript(fn_script) => {
                if arg_count != fn_script.arity {
                    return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
                }

                (fn_script.bytecode.clone(), None)
            }
            Value::Class(class) => {
                let class = class.clone();
                let object = Object::new(class.instance_type_id(), Value::Class(class.clone()));

                // TODO: Call constructor (if one exists).

                if let Some(_constructor) = &class.constructor() {
                    todo!("Actually handle the constructor case.")
                } else {
                    if arg_count > 0 {
                        return Err(RuntimeError::InvalidFunctionArgs(0, arg_count));
                    }

                    self.stack.release_slots(1);
                }

                self.stack.push(Value::Object(object));
                return Ok(());
            }
            Value::FnNative(fn_native) => {
                let fn_native = fn_native.clone();
                // NOTE: Include the function/receiver slot as the first parameter to the native function call.
                let mut args = self.stack.pop_count(arg_count + 1);

                if let Some(receiver) = receiver {
                    args[0] = receiver;
                }

                let result = fn_native.call(self, &args)?;

                self.stack.push(result);

                return Ok(());
            }
            _ => return Err(RuntimeError::NotAFunction),
        };

        let slots = bytecode.slot_count();
        let reserved = slots - arg_count;
        // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
        let stack_frame = self.stack.reserve_slots(reserved);
        // NOTE: Calling convention includes an extra parameter. This parameter is the function itself for bare functions
        // and the receiver for methods.
        let stack_frame = stack_frame.prepend(arg_count + 1);

        if let Some(receiver) = receiver {
            self.stack[stack_frame][0] = receiver;
        }

        let result = self.execute_bytecode(&bytecode, stack_frame, closure)?;

        // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
        self.stack.release_slots(reserved + arg_count + 1);
        self.stack.push(result);

        Ok(())
    }

    fn load_module(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let path_slot = cursor.read_u8() as usize;
        let path = bytecode.constants()[path_slot].as_str()?;
        let module = self.module_loader.load_module(path)?;
        let module = match self.loaded_modules.entry(module.id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let export = Value::Object(Object::new(TypeId::new(None, path, "#export"), None));
                entry.insert(export.clone());
                self.run_module(module.bytecode, export)?
            }
        };

        self.stack.push(module);

        Ok(())
    }
}
