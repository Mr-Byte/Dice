use crate::{error::RuntimeError, stack::Stack};
use dice_core::operator::ADD;
use dice_core::{
    bytecode::{instruction::Instruction, Bytecode, BytecodeCursor},
    upvalue::{Upvalue, UpvalueState},
    value::{FnClosure, FnNative, NativeFn, Object, Value},
};
use std::collections::hash_map::Entry;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

#[derive(Default)]
pub struct Runtime {
    stack: Stack,
    open_upvalues: VecDeque<Upvalue>,
    globals: HashMap<String, Value>,
}

// TODO: Split off the main interpreter loop and associated functions into their own module
// TODO: Add a public function to allow native code to execute scripted functions
//     fn run_fn(target: Value, args: &[Value]) -> Result<Value, Error>
impl Runtime {
    pub fn run_bytecode(&mut self, bytecode: Bytecode) -> Result<Value, RuntimeError> {
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }

    pub fn register_native_fn(&mut self, name: String, native_fn: NativeFn) {
        self.globals.insert(name, Value::FnNative(FnNative::new(native_fn)));
    }

    fn execute_bytecode(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: Range<usize>,
        mut closure: Option<FnClosure>,
    ) -> Result<Value, RuntimeError> {
        let initial_stack_depth = self.stack.len();
        let mut cursor = bytecode.cursor();

        while let Some(instruction) = cursor.read_instruction() {
            match instruction {
                Instruction::PUSH_NONE => self.stack.push(Value::Null),
                Instruction::PUSH_UNIT => self.stack.push(Value::Unit),
                Instruction::PUSH_FALSE => self.stack.push(Value::Bool(false)),
                Instruction::PUSH_TRUE => self.stack.push(Value::Bool(true)),
                Instruction::PUSH_I0 => self.stack.push(Value::Int(0)),
                Instruction::PUSH_I1 => self.stack.push(Value::Int(1)),
                Instruction::PUSH_F0 => self.stack.push(Value::Float(0.0)),
                Instruction::PUSH_F1 => self.stack.push(Value::Float(1.0)),
                Instruction::PUSH_CONST => self.push_const(bytecode, &mut cursor),
                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::DUP => {
                    let value = self.stack.peek(0).clone();
                    self.stack.push(value);
                }
                Instruction::SWAP => {
                    let slots = self.stack.len() - 2..self.stack.len();
                    let (first, second) = self.stack.slots(slots).split_at_mut(1);
                    std::mem::swap(&mut second[0], &mut first[0]);
                }
                Instruction::CREATE_LIST => self.create_list(&mut cursor),
                Instruction::CREATE_OBJECT => self.create_object(&mut cursor),

                Instruction::NEG => self.neg(),
                Instruction::NOT => self.not()?,

                Instruction::MUL => arithmetic_op!(self.stack, OP_MUL),
                Instruction::DIV => arithmetic_op!(self.stack, OP_DIV),
                Instruction::REM => arithmetic_op!(self.stack, OP_REM),
                Instruction::ADD => self.add()?,
                Instruction::SUB => arithmetic_op!(self.stack, OP_SUB),

                Instruction::GT => comparison_op!(self.stack, OP_GT),
                Instruction::GTE => comparison_op!(self.stack, OP_GTE),
                Instruction::LT => comparison_op!(self.stack, OP_LT),
                Instruction::LTE => comparison_op!(self.stack, OP_LTE),
                Instruction::EQ => comparison_op!(self.stack, OP_EQ),
                Instruction::NEQ => comparison_op!(self.stack, OP_NEQ),

                Instruction::JUMP => {
                    let offset = cursor.read_offset();
                    cursor.offset_position(offset);
                }
                Instruction::JUMP_IF_FALSE => self.jump_if_false(&mut cursor)?,

                Instruction::LOAD_LOCAL => self.load_local(stack_frame.clone(), &mut cursor),
                Instruction::STORE_LOCAL => self.store_local(stack_frame.clone(), &mut cursor),
                Instruction::LOAD_UPVALUE => self.load_upvalue(&mut closure, &mut cursor),
                Instruction::STORE_UPVALUE => self.store_upvalue(&mut closure, &mut cursor),
                Instruction::CLOSE_UPVALUE => self.close_upvalue(&stack_frame, &mut cursor),
                Instruction::LOAD_GLOBAL => self.load_global(bytecode, &mut cursor)?,
                Instruction::STORE_GLOBAL => self.store_global(bytecode, &mut cursor)?,
                Instruction::LOAD_FIELD => self.load_field(bytecode, &mut cursor)?,
                Instruction::STORE_FIELD => self.store_field(bytecode, &mut cursor)?,
                Instruction::LOAD_INDEX => self.load_index()?,
                Instruction::STORE_INDEX => self.store_index()?,

                Instruction::CLOSURE => self.closure(bytecode, &stack_frame, &mut closure, &mut cursor)?,
                Instruction::CALL => {
                    let arg_count = cursor.read_u8() as usize;
                    self.call(arg_count)?;
                }
                Instruction::RETURN => break,

                Instruction::ASSERT_BOOL => {
                    if !self.stack.peek(0).is_bool() {
                        return Err(RuntimeError::Aborted(String::from(
                            "Right hand side must evaluate to a boolean.",
                        )));
                    }
                }

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
    fn not(&mut self) -> Result<(), RuntimeError> {
        match self.stack.peek(0) {
            Value::Bool(value) => *value = !*value,
            _ => return Err(RuntimeError::Aborted(String::from("LHS must be a boolean value."))),
        }

        Ok(())
    }

    #[inline]
    fn neg(&mut self) {
        match self.stack.peek(0) {
            Value::Int(value) => *value = -*value,
            Value::Float(value) => *value = -*value,
            _ => todo!(),
        }
    }

    #[inline]
    fn add(&mut self) -> Result<(), RuntimeError> {
        match (self.stack.pop(), self.stack.peek(0)) {
            (Value::Int(rhs), Value::Int(lhs)) => *lhs = *lhs + rhs,
            (Value::Float(rhs), Value::Float(lhs)) => *lhs = *lhs + rhs,
            (rhs, _) => match self.globals.get(ADD) {
                Some(value) => {
                    let lhs = self.stack.pop();
                    self.stack.push(value.clone());
                    self.stack.push(lhs);
                    self.stack.push(rhs);
                    self.call(2)?;
                }
                None => todo!(),
            },
        }

        Ok(())
    }

    fn create_list(&mut self, cursor: &mut BytecodeCursor) {
        let count = cursor.read_u8() as usize;
        let items = self.stack.pop_count(count);

        self.stack.push(Value::List(items.to_vec().into()));
    }

    fn create_object(&mut self, cursor: &mut BytecodeCursor) {
        let type_id = cursor.read_type_id();
        let object = Object::new(type_id);

        self.stack.push(Value::Object(object));
    }

    fn push_const(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) {
        let const_pos = cursor.read_u8() as usize;
        let value = bytecode.constants()[const_pos].clone();
        self.stack.push(value);
    }

    fn jump_if_false(&mut self, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let offset = cursor.read_offset();
        let value = match self.stack.pop() {
            Value::Bool(value) => value,
            _ => {
                return Err(RuntimeError::Aborted(String::from(
                    "JUMP_IF_FALSE requires a boolean operand.",
                )));
            }
        };

        if !value {
            cursor.offset_position(offset)
        }

        Ok(())
    }

    fn load_local(&mut self, stack_frame: Range<usize>, cursor: &mut BytecodeCursor) {
        // TODO Bounds check the slot?
        let slot = cursor.read_u8() as usize;
        let frame = self.stack.slots(stack_frame);
        let value = frame[slot].clone();
        self.stack.push(value);
    }

    fn store_local(&mut self, stack_frame: Range<usize>, cursor: &mut BytecodeCursor) {
        let value = self.stack.pop();
        let slot = cursor.read_u8() as usize;

        self.stack.slots(stack_frame)[slot] = value.clone();
        self.stack.push(value);
    }

    fn load_upvalue(&mut self, closure: &mut Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure.as_mut() {
            let upvalue_slot = cursor.read_u8() as usize;
            let mut upvalue = closure.upvalues[upvalue_slot].clone();
            let value = match &*upvalue.state() {
                UpvalueState::Open(slot) => self.stack.slot(*slot).clone(),
                UpvalueState::Closed(value) => value.clone(),
            };

            self.stack.push(value);
        } else {
            unreachable!("LOAD_UPVALUE used in non-closure context.")
        }
    }

    fn store_upvalue(&mut self, closure: &mut Option<FnClosure>, cursor: &mut BytecodeCursor) {
        if let Some(closure) = closure.as_mut() {
            let upvalue_slot = cursor.read_u8() as usize;
            let mut upvalue = closure.upvalues[upvalue_slot].clone();
            let value = self.stack.pop();
            let result = match &mut *upvalue.state() {
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

    fn close_upvalue(&mut self, stack_frame: &Range<usize>, cursor: &mut BytecodeCursor) {
        let offset = cursor.read_u8() as usize;
        let value = std::mem::replace(&mut self.stack.slots(stack_frame.clone())[offset], Value::Null);
        let offset = stack_frame.start + offset;
        let mut found_index = None;

        for (index, upvalue) in self.open_upvalues.iter_mut().enumerate() {
            if let UpvalueState::Open(upvalue_offset) = &*upvalue.state() {
                if *upvalue_offset == offset {
                    found_index = Some(index);
                }
            }
        }

        if let Some(index) = found_index {
            if let Some(mut upvalue) = self.open_upvalues.remove(index) {
                upvalue.close(value);
            }
        }
    }

    fn store_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;
        let value = &bytecode.constants()[const_pos];

        if let Value::String(global) = value {
            let global_name = (**global).clone();
            let global = self.stack.pop();

            match self.globals.entry(global_name) {
                Entry::Occupied(_) => todo!("Return error that global already exists."),
                Entry::Vacant(entry) => {
                    entry.insert(global);
                }
            }
        } else {
            return Err(RuntimeError::InvalidGlobalNameType);
        }

        Ok(())
    }

    fn load_global(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let const_pos = cursor.read_u8() as usize;
        let value = &bytecode.constants()[const_pos];

        if let Value::String(global) = value {
            let global = (**global).clone();
            let value = self
                .globals
                .get(&global)
                .cloned()
                .ok_or_else(|| RuntimeError::VariableNotFound(global))?;

            self.stack.push(value);
        } else {
            return Err(RuntimeError::InvalidGlobalNameType);
        }

        Ok(())
    }

    fn store_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let key_index = cursor.read_u8() as usize;
        match &bytecode.constants()[key_index] {
            Value::String(key) => {
                let value = self.stack.pop();
                match self.stack.pop() {
                    Value::Object(object) => {
                        object.fields_mut().insert((**key).clone(), value.clone());
                        self.stack.push(value);
                    }
                    _ => todo!("Throw an error if the target is not an object."),
                }
            }
            _ => todo!("Throw an error if the key value is not a string."),
        }

        Ok(())
    }

    fn load_field(&mut self, bytecode: &Bytecode, cursor: &mut BytecodeCursor) -> Result<(), RuntimeError> {
        let key_index = cursor.read_u8() as usize;
        let value = match &bytecode.constants()[key_index] {
            Value::String(key) => match self.stack.pop() {
                Value::Object(object) => match object.fields().get(&**key) {
                    Some(field) => field.clone(),
                    None => Value::Null,
                },
                _ => todo!("Throw an error if the target is not an object."),
            },
            _ => todo!("Throw an error if the key value is not a string."),
        };

        self.stack.push(value);

        Ok(())
    }

    #[inline]
    fn store_index(&mut self) -> Result<(), RuntimeError> {
        let value = self.stack.pop();
        let index = self.stack.pop();
        let target = self.stack.peek(0);

        match target {
            Value::Object(object) => match index {
                Value::String(field) => {
                    object.fields_mut().insert((*field).clone(), value.clone());
                    *target = value;
                }
                _ => todo!("Return invalid key type"),
            },
            Value::List(list) => match index {
                Value::Int(index) => {
                    list.elements_mut()[index as usize] = value.clone();
                    *target = value;
                }
                _ => todo!("Return invalid key type"),
            },
            _ => todo!("Return invalid index target error."),
        };

        Ok(())
    }

    #[inline]
    fn load_index(&mut self) -> Result<(), RuntimeError> {
        let index = self.stack.pop();
        let target = self.stack.peek(0);

        let result = match target {
            Value::Object(object) => match index {
                Value::String(field) => object.fields().get(&**field).cloned(),
                _ => todo!("Return invalid key type"),
            },
            Value::List(list) => match index {
                Value::Int(index) => list.elements().get(index as usize).cloned(),
                _ => todo!("Return invalid key type"),
            },
            _ => todo!("Return invalid index target error."),
        };

        *target = result.unwrap_or(Value::Null);

        Ok(())
    }

    fn closure(
        &mut self,
        bytecode: &Bytecode,
        stack_frame: &Range<usize>,
        closure: &mut Option<FnClosure>,
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
                        let upvalue = Upvalue::new_open(stack_frame.start + index);
                        self.open_upvalues.push_back(upvalue.clone());
                        upvalues.push(upvalue);
                    } else if let Some(closure) = closure.as_mut() {
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

    // TODO: Replace this mutually recursive call with an execution stack to prevent the thread's stack from overflowing.
    fn call(&mut self, arg_count: usize) -> Result<(), RuntimeError> {
        let target = self.stack.peek(arg_count);
        let (bytecode, closure) = match target {
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
            Value::FnNative(fn_native) => {
                let fn_native = fn_native.clone();
                let mut args = self.stack.pop_count(arg_count);
                let result = fn_native.call(self, &mut args)?;

                self.stack.release_slots(1);
                self.stack.push(result);

                return Ok(());
            }
            _ => return Err(RuntimeError::NotAFunction),
        };

        let slots = bytecode.slot_count();
        let reserved = slots - arg_count;
        // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
        let stack_frame = self.stack.reserve_slots(reserved);
        let stack_frame = (stack_frame.start - arg_count)..stack_frame.end;
        let result = self.execute_bytecode(&bytecode, stack_frame, closure)?;

        // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
        self.stack.release_slots(reserved + arg_count + 1);
        self.stack.push(result);

        Ok(())
    }
}

impl dice_core::runtime::Runtime for Runtime {}
