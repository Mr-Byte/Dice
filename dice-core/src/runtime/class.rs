use crate::value::{Class, NativeFn, Value};

#[derive(Clone, Debug)]
pub struct ClassBuilder {
    class: Class,
}

impl ClassBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            class: Class::new(name.into()),
        }
    }

    pub fn derive(&self, name: &str) -> Self {
        Self {
            class: Class::with_base(name.into(), self.class.clone()),
        }
    }

    pub fn class(&self) -> Class {
        self.class.clone()
    }

    pub fn register_native_method(&mut self, name: &str, method: NativeFn) {
        self.class
            .methods_mut()
            .insert(name.into(), Value::with_native_fn(method));
    }

    pub fn register_native_static_property(&mut self, name: &str, value: Value) {
        self.class.fields_mut().insert(name.into(), value);
    }
}
