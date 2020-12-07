use dice_compiler::compiler::Compiler;
use dice_core::source::{Source, SourceKind};
use dice_runtime::runtime;

pub use dice_core::{protocol, runtime::Runtime, value};

pub struct Dice {
    runtime: runtime::Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: impl Into<String>) -> Result<value::Value, String> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile(&source);

        match bytecode {
            Ok(bytecode) => {
                let value = self.runtime.run_bytecode(bytecode).expect("Error conversion.");

                Ok(value)
            }
            Err(error) => {
                let error = source.format_error(&error);
                Err(error)
            }
        }
    }

    pub fn disassemble_script(&self, input: impl Into<String>) -> Result<String, String> {
        let source = Source::new(input.into(), SourceKind::Script);
        let bytecode = Compiler::compile(&source);

        match bytecode {
            Ok(bytecode) => Ok(bytecode.to_string()),
            Err(error) => {
                let error = source.format_error(&error);
                Err(error)
            }
        }
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
