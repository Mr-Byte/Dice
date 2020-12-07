use dice_compiler::compiler::Compiler;
use dice_core::source::{Source, SourceKind};
use dice_error::compiler_error::CompilerError;
use dice_runtime::runtime;

pub use dice_core::{protocol, runtime::Runtime, value};

pub struct Dice {
    runtime: runtime::Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: impl Into<String>) -> Result<value::Value, DiceError> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile(source)?;
        let value = self.runtime.run_bytecode(bytecode).expect("Error conversion.");

        Ok(value)
    }

    pub fn disassemble_script(&self, input: impl Into<String>) -> Result<String, DiceError> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile(source)?;
        Ok(bytecode.to_string())
    }

    pub fn runtime(&mut self) -> &mut impl Runtime {
        &mut self.runtime
    }

    // pub fn register_native_fn(&mut self, name: &str, native_fn: NativeFn) {
    //     self.runtime.function(name, native_fn);
    // }
}

impl Default for Dice {
    fn default() -> Self {
        let runtime = runtime::Runtime::default();

        Self { runtime }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DiceError {
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
}
