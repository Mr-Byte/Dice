use dice_compiler::{
    compiler::{CompilationKind, Compiler},
    error::CompilerError,
};
use dice_core::value::NativeFn;
pub use dice_core::{
    runtime::{NativeError, Runtime},
    value::Value,
};
pub use dice_runtime::error::RuntimeError;
use dice_runtime::runtime;

pub struct Dice {
    runtime: runtime::Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: &str) -> Result<Value, DiceError> {
        let bytecode = Compiler::compile_str(input, CompilationKind::Script)?;
        let value = self.runtime.run_bytecode(bytecode)?;

        Ok(value)
    }

    pub fn disassemble_script(&self, input: &str) -> Result<String, DiceError> {
        let bytecode = Compiler::compile_str(input, CompilationKind::Script)?;
        Ok(bytecode.to_string())
    }

    pub fn register_native_fn(&mut self, name: impl Into<String>, native_fn: NativeFn) {
        self.runtime.register_native_fn(name.into(), native_fn);
    }
}

impl Default for Dice {
    fn default() -> Self {
        let runtime = runtime::Runtime::with_file_module_loader();

        // TODO: Load the prelude into the runtime.

        Self { runtime }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DiceError {
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
}
