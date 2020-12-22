use crate::{
    module::{file_loader::FileModuleLoader, ModuleLoader},
    stack::Stack,
};
use dice_core::{
    bytecode::Bytecode,
    error::{
        codes::{GLOBAL_ALREADY_EXISTS, MODULE_ALREADY_EXISTS},
        Error,
    },
    tags,
    upvalue::Upvalue,
    value::{Class, Object, Value, ValueKind, ValueMap},
};
use std::{
    collections::{HashMap, VecDeque},
    hash::BuildHasherDefault,
};
use wyhash::WyHash;

pub struct Runtime<L = FileModuleLoader>
where
    L: ModuleLoader,
{
    pub(super) stack: Stack,
    pub(crate) open_upvalues: VecDeque<Upvalue>,
    pub(crate) globals: ValueMap,
    pub(crate) loaded_modules: ValueMap,
    pub(crate) module_loader: L,
    pub(crate) any_class: Class,
    pub(crate) module_class: Class,
    pub(crate) value_class_mapping: HashMap<ValueKind, Class, BuildHasherDefault<WyHash>>,
}

impl<L> Default for Runtime<L>
where
    L: ModuleLoader,
{
    fn default() -> Self {
        let any_class = Self::new_any_class();
        let module_class = Self::new_module_class(&any_class);
        let mut globals: ValueMap = ValueMap::default();
        globals.insert(any_class.name(), Value::Class(any_class.clone()));
        globals.insert(module_class.name(), Value::Class(module_class.clone()));

        let mut runtime = Self {
            stack: Default::default(),
            open_upvalues: Default::default(),
            loaded_modules: Default::default(),
            module_loader: Default::default(),
            value_class_mapping: Default::default(),
            globals,
            any_class,
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
    pub fn run_bytecode(&mut self, bytecode: Bytecode) -> Result<Value, Error> {
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        let result = self.execute_bytecode(&bytecode, stack_frame, None)?;
        self.stack.release_stack_frame(stack_frame);

        Ok(result)
    }

    pub(super) fn run_module(&mut self, bytecode: Bytecode, export: Value) -> Result<Value, Error> {
        let stack_frame = self.stack.reserve_slots(bytecode.slot_count());
        self.stack[stack_frame.start()] = export;
        let result = self.execute_bytecode(&bytecode, stack_frame, None)?;
        self.stack.release_stack_frame(stack_frame);

        Ok(result)
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
    fn new_module(&mut self, name: &str) -> Result<Object, Error> {
        let module = Object::new(None);

        if self
            .loaded_modules
            .insert(name.into(), Value::Object(module.clone()))
            .is_some()
        {
            return Err(Error::new(MODULE_ALREADY_EXISTS).with_tags(tags! {
                name => name.to_string()
            }));
        }

        Ok(module)
    }

    fn new_class(&mut self, name: &str) -> Result<Class, Error> {
        let class = Class::with_base(name.into(), self.any_class.clone());

        Ok(class)
    }

    fn new_object(&mut self) -> Result<Object, Error> {
        let object = Object::new(self.any_class.clone());

        Ok(object)
    }

    fn load_prelude(&mut self, path: &str) -> Result<(), Error> {
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

    fn add_global(&mut self, name: &str, value: Value) -> Result<(), Error> {
        if self.globals.insert(name.into(), value).is_some() {
            return Err(Error::new(GLOBAL_ALREADY_EXISTS).with_tags(tags! {
                name => name.to_string()
            }));
        }

        Ok(())
    }

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, Error> {
        let arg_count = args.len();
        self.stack.push(target);
        self.stack.push_multiple(args);
        self.call_fn(arg_count)?;

        Ok(self.stack.pop())
    }

    fn any_class(&self) -> Result<Class, Error> {
        Ok(self.any_class.clone())
    }

    fn class_of(&self, value: &Value) -> Result<Class, Error> {
        let result = value
            .as_object()
            .ok()
            .and_then(|object| object.class())
            .or_else(|| self.value_class_mapping.get(&value.kind()).cloned())
            .unwrap_or_else(|| self.any_class.clone());

        Ok(result)
    }

    fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, Error> {
        value
            .as_object()
            .ok()
            .and_then(|object| object.class())
            .or_else(|| self.value_class_mapping.get(&value.kind()).cloned())
            .map_or(Ok(false), |instance_class| Ok(instance_class.is_class(&class)))
    }
}
