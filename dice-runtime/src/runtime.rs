use crate::{
    module::{file_loader::FileModuleLoader, ModuleLoader},
    stack::Stack,
};
use dice_core::{
    bytecode::Bytecode,
    upvalue::Upvalue,
    value::{Class, Object, Value, ValueKind, ValueMap},
};
use dice_error::runtime_error::RuntimeError;
use std::{
    collections::{HashMap, VecDeque},
    hash::BuildHasherDefault,
};
use wyhash::WyHash;

pub struct Runtime<L = FileModuleLoader>
where
    L: ModuleLoader,
{
    pub(crate) stack: Stack,
    pub(crate) open_upvalues: VecDeque<Upvalue>,
    pub(crate) globals: ValueMap,
    pub(crate) loaded_modules: ValueMap,
    pub(crate) module_loader: L,
    pub(crate) object_class: Class,
    pub(crate) module_class: Class,
    pub(crate) value_class_mapping: HashMap<ValueKind, Class, BuildHasherDefault<WyHash>>,
}

impl<L> Default for Runtime<L>
where
    L: ModuleLoader,
{
    fn default() -> Self {
        let object_class = Self::new_object_class();
        let module_class = Self::new_module_class(&object_class);
        let mut globals: ValueMap = ValueMap::default();
        globals.insert(object_class.name(), Value::Class(object_class.clone()));
        globals.insert(module_class.name(), Value::Class(module_class.clone()));

        let mut runtime = Self {
            stack: Default::default(),
            open_upvalues: Default::default(),
            loaded_modules: Default::default(),
            module_loader: Default::default(),
            value_class_mapping: Default::default(),
            globals,
            object_class,
            module_class,
        };

        runtime.register_known_types();
        runtime
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

    pub(super) fn set_value_class(&mut self, value_kind: ValueKind, class: Class) {
        self.value_class_mapping.insert(value_kind, class.clone());
        self.globals.insert(class.name(), Value::Class(class));
    }
}

impl<L> dice_core::runtime::Runtime for Runtime<L>
where
    L: ModuleLoader,
{
    fn new_module(&mut self, name: &str) -> Result<Object, RuntimeError> {
        let module = Object::new(None);

        if self
            .loaded_modules
            .insert(name.into(), Value::Object(module.clone()))
            .is_some()
        {
            return Err(RuntimeError::Aborted(String::from("Module already registered.")));
        }

        Ok(module)
    }

    fn new_class(&mut self, name: &str) -> Result<Class, RuntimeError> {
        let class = Class::with_base(name.into(), self.object_class.clone());

        Ok(class)
    }

    fn new_object(&mut self) -> Result<Object, RuntimeError> {
        let object = Object::new(self.object_class.clone());

        Ok(object)
    }

    fn load_prelude(&mut self, path: &str) -> Result<(), RuntimeError> {
        // TODO: Clean this up unify module loading with the runtime's own module loading process.
        let module = self.module_loader.load_module(path.into())?;
        let prelude = Value::Object(Object::new(self.module_class.clone()));
        // NOTE: Add the loaded prelude module as a registered module.
        self.loaded_modules.insert(module.id.clone(), prelude.clone());

        let prelude = self.run_module(module.bytecode, prelude)?;

        for (name, value) in &*prelude.as_object()?.fields() {
            self.globals.entry(name.clone()).or_insert_with(|| value.clone());
        }

        Ok(())
    }

    fn add_global(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.globals.insert(name.into(), value).is_some() {
            // TODO: Create a separate error variant for this.
            return Err(RuntimeError::Aborted(String::from("Global already registered.")));
        }

        Ok(())
    }

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError> {
        self.stack.push(target);
        self.stack.push_slice(args);
        self.call_fn(args.len())?;

        Ok(self.stack.pop())
    }
}
