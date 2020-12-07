use crate::value::{Class, Object, Value};

pub trait Runtime {
    fn new_module(&mut self, name: &str) -> Result<Object, ()>;
    fn new_class(&mut self, name: &str) -> Result<Class, ()>;
    fn new_object(&mut self) -> Result<Object, ()>;

    fn load_prelude(&mut self, path: &str) -> Result<(), ()>;
    fn add_global(&mut self, name: &str, value: Value) -> Result<(), ()>;

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, ()>;

    fn any_class(&self) -> Result<Class, ()>;
    fn class_of(&self, value: &Value) -> Result<Class, ()>;
    fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, ()>;
}
