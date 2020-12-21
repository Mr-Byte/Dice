use dice_compiler::compiler::Compiler;
use dice_core::source::{Source, SourceKind};
use dice_runtime::runtime;

pub use dice_core::tags;
pub use dice_core::{error, protocol, runtime::Runtime, value};

pub struct Dice {
    runtime: runtime::Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: impl Into<String>) -> Result<value::Value, error::Error> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile_source(source)?;
        let value = self.runtime.run_bytecode(bytecode)?;

        Ok(value)
    }

    pub fn disassemble_script(&self, input: impl Into<String>) -> Result<String, error::Error> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile_source(source)?;

        Ok(bytecode.to_string())
    }

    pub fn runtime(&mut self) -> &mut impl Runtime {
        &mut self.runtime
    }
}

impl Default for Dice {
    fn default() -> Self {
        let runtime = runtime::Runtime::default();

        Self { runtime }
    }
}
