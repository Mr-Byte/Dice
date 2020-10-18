mod class;
mod module;

use crate::value::{NativeFn, Symbol, Value};
use dice_error::runtime_error::RuntimeError;

pub use class::*;
pub use module::*;

pub trait Runtime {
    /// Load a script and register its exported items as global items.
    fn load_prelude(&mut self, path: &str) -> Result<(), RuntimeError>;

    /// Call the target function with the provided arguments.
    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, RuntimeError>;

    /// Register a global native function.
    fn register_native_function(&mut self, name: &str, native_fn: NativeFn);

    /// Create a new native module.
    fn new_module(&mut self, name: Symbol) -> Result<Module, RuntimeError>;

    /// Create a new global class.
    fn new_class(&mut self, name: Symbol) -> Result<Class, RuntimeError>;
}
