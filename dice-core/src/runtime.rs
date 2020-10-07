use crate::value::{NativeFn, Value};
use dice_error::runtime_error::RuntimeError;

pub trait Runtime {
    fn function(&mut self, name: &str, native_fn: NativeFn);
    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError>;
    fn module(&mut self, name: &str) -> Result<Module, RuntimeError>;
}

pub struct Module;
