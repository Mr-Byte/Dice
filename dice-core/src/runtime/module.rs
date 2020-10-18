use crate::runtime::Class;
use crate::value::{NativeFn, Object, Value};
use gc::{Finalize, Trace};

#[derive(Default, Clone, Debug, Trace, Finalize)]
pub struct Module {
    module_object: Object,
}

impl Module {
    pub fn object(&self) -> Object {
        self.module_object.clone()
    }

    pub fn new_class(&self) -> Class {
        todo!("Emit a new class.")
    }

    pub fn register_native_function(&mut self, name: &str, native_fn: impl Into<NativeFn>) {
        self.module_object
            .fields_mut()
            .insert(name.into(), Value::with_native_fn(native_fn));
    }
}
