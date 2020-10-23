use crate::{
    runtime::ClassBuilder,
    value::{NativeFn, Object, Value},
};

#[derive(Default, Clone, Debug)]
pub struct ModuleBuilder {
    module_object: Object,
}

impl ModuleBuilder {
    pub fn object(&self) -> Object {
        self.module_object.clone()
    }

    pub fn new_class(&self, name: &str) -> ClassBuilder {
        let builder = ClassBuilder::new(name);
        self.module_object
            .fields_mut()
            .insert(name.into(), Value::Class(builder.class()));

        builder
    }

    pub fn register_native_function(&mut self, name: &str, native_fn: NativeFn) {
        self.module_object
            .fields_mut()
            .insert(name.into(), Value::with_native_fn(native_fn));
    }
}
