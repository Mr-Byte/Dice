use dice_compiler::compiler::{CompilationKind, Compiler};
use dice_compiler::error::CompilerError;
pub use dice_core::runtime::NativeError;
pub use dice_core::runtime::Runtime;
use dice_core::value::NativeFn;
pub use dice_core::value::Value;
pub use dice_runtime::error::RuntimeError;
use dice_syntax::SyntaxError;

#[derive(Default)]
pub struct Dice {
    runtime: dice_runtime::runtime::Runtime,
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

#[derive(thiserror::Error, Debug)]
pub enum DiceError {
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
}
