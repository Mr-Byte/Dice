use crate::value::Class;
use gc::{Finalize, Trace};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct ClassBuilder {
    class: Class,
}

impl ClassBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            class: Class::new(name.into()),
        }
    }

    pub fn class(&self) -> Class {
        self.class.clone()
    }

    pub fn register_native_method() {}
}
