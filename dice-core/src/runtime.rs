use crate::value::{Class, Object, Value};
use dice_error::runtime_error::RuntimeError;

pub trait Runtime {
    fn new_module(&mut self, name: &str) -> Result<Object, RuntimeError>;
    fn new_class(&mut self, name: &str) -> Result<Class, RuntimeError>;
    fn new_object(&mut self) -> Result<Object, RuntimeError>;

    fn load_prelude(&mut self, path: &str) -> Result<(), RuntimeError>;
    fn add_global(&mut self, name: &str, value: Value) -> Result<(), RuntimeError>;

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError>;

    fn any_class(&self) -> Result<Class, RuntimeError>;
    fn class_of(&self, value: &Value) -> Result<Class, RuntimeError>;
    fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, RuntimeError>;
}
