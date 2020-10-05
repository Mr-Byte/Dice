mod interpreter;
mod stack;

use crate::{
    error::RuntimeError,
    module_loader::{FileModuleLoader, ModuleId, ModuleLoader},
    runtime::stack::Stack,
};
use dice_core::runtime::NativeError;
use dice_core::{
    bytecode::Bytecode,
    upvalue::{Upvalue, UpvalueState},
    value::{FnNative, NativeFn, Value},
};
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct Runtime<L = FileModuleLoader>
where
    L: ModuleLoader,
{
    stack: Stack,
    open_upvalues: VecDeque<Upvalue>,
    globals: HashMap<String, Value>,
    loaded_modules: HashMap<ModuleId, Value>,
    module_loader: L,
}

impl<L> Runtime<L>
where
    L: ModuleLoader + Default,
{
    pub fn run_bytecode(&mut self, bytecode: Bytecode) -> Result<Value, RuntimeError> {
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }

    pub(super) fn run_module(&mut self, bytecode: Bytecode, export: Value) -> Result<Value, RuntimeError> {
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        *self.stack.slot(stack_frame.start) = export;
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }

    pub(super) fn find_open_upvalue(&self, offset: usize) -> Option<(usize, Upvalue)> {
        let mut found_upvalue = None;

        for (index, upvalue) in self.open_upvalues.iter().enumerate() {
            if let UpvalueState::Open(upvalue_offset) = &*upvalue.state() {
                if *upvalue_offset == offset {
                    found_upvalue = Some((index, upvalue.clone()));
                }
            }
        }

        found_upvalue
    }
}

impl<L> dice_core::runtime::Runtime for Runtime<L>
where
    L: ModuleLoader,
{
    fn register_native_fn(&mut self, name: &str, native_fn: NativeFn) {
        self.globals
            .insert(name.to_owned(), Value::FnNative(FnNative::new(native_fn)));
    }

    fn call_fn(&mut self, target: Value, args: &[Value]) -> Result<Value, NativeError> {
        self.stack.push(target);
        self.stack.push_slice(args);

        Self::call_fn(self, args.len()).expect("Fix error handling.");
        Ok(self.stack.pop())
    }
}
