mod call_frame;
mod helper;
mod interpreter;
mod stack;

use crate::{
    module::{file_loader::FileModuleLoader, ModuleLoader},
    runtime::stack::Stack,
};
use dice_core::runtime::Class;
use dice_core::value::Symbol;
use dice_core::{
    bytecode::Bytecode,
    id::type_id::TypeId,
    runtime::Module,
    upvalue::Upvalue,
    value::{FnNative, NativeFn, Object, Value, ValueMap},
};
use dice_error::runtime_error::RuntimeError;
use std::collections::VecDeque;

pub struct Runtime<L = FileModuleLoader>
where
    L: ModuleLoader,
{
    stack: Stack,
    open_upvalues: VecDeque<Upvalue>,
    globals: ValueMap,
    loaded_modules: ValueMap,
    module_loader: L,
}

impl<L> Default for Runtime<L>
where
    L: ModuleLoader,
{
    fn default() -> Self {
        Self {
            stack: Default::default(),
            open_upvalues: Default::default(),
            globals: Default::default(),
            loaded_modules: Default::default(),
            module_loader: Default::default(),
        }
    }
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
        self.stack[stack_frame.start()] = export;
        let result = self.execute_bytecode(&bytecode, stack_frame, None);
        self.stack.release_slots(bytecode.slot_count());

        Ok(result?)
    }
}

impl<L> dice_core::runtime::Runtime for Runtime<L>
where
    L: ModuleLoader,
{
    fn load_prelude(&mut self, path: &str) -> Result<(), RuntimeError> {
        let module = self.module_loader.load_module(path.into())?;
        let prelude = Value::Object(Object::new(TypeId::default(), None));
        // NOTE: Add the loaded prelude module as a registered module.
        self.loaded_modules.insert(module.id.clone(), prelude.clone());

        let prelude = self.run_module(module.bytecode, prelude)?;

        for (name, value) in &*prelude.as_object()?.fields() {
            self.globals.entry(name.clone()).or_insert_with(|| value.clone());
        }

        Ok(())
    }

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError> {
        self.stack.push(target);
        self.stack.push_slice(args);
        self.call_fn(args.len())?;

        Ok(self.stack.pop())
    }

    fn register_native_function(&mut self, name: &str, native_fn: NativeFn) {
        self.globals
            .insert(name.into(), Value::FnNative(FnNative::new(native_fn)));
    }

    fn new_module(&mut self, _name: Symbol) -> Result<Module, RuntimeError> {
        unimplemented!()
    }

    fn new_class(&mut self, _name: Symbol) -> Result<Class, RuntimeError> {
        unimplemented!()
    }
}
