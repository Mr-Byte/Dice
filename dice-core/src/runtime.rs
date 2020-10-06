use crate::value::{NativeFn, Value};
use dice_error::runtime_error::RuntimeError;

pub trait Runtime {
    fn add_native_fn(&mut self, name: &str, native_fn: NativeFn);
    fn call_fn(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError>;
}
