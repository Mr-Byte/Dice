use crate::{
    error::Error,
    value::{Class, Object, Value},
};

pub trait Runtime {
    fn new_module(&mut self, name: &str) -> Result<Object, Error>;
    fn new_class(&mut self, name: &str) -> Result<Class, Error>;
    fn new_object(&mut self) -> Result<Object, Error>;

    fn load_prelude(&mut self, path: &str) -> Result<(), Error>;
    fn add_global(&mut self, name: &str, value: Value) -> Result<(), Error>;

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, Error>;

    fn any_class(&self) -> Result<Class, Error>;
    fn class_of(&self, value: &Value) -> Result<Class, Error>;
    fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, Error>;
}
