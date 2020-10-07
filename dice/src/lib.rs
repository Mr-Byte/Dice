use dice_compiler::compiler::{CompilationKind, Compiler};
use dice_core::value::NativeFn;
pub use dice_core::{runtime::Runtime, value::Value};
use dice_error::compiler_error::CompilerError;
use dice_runtime::runtime;

pub use dice_error::runtime_error::RuntimeError;

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

    pub fn register_native_fn(&mut self, name: &str, native_fn: NativeFn) {
        self.runtime.function(name, native_fn);
    }
}

impl Default for Dice {
    fn default() -> Self {
        let runtime = runtime::Runtime::default();

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
