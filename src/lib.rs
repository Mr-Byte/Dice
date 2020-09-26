use crate::common::span::Span;
pub use compiler::error::CompilerError;
use compiler::{CompilationKind, Compiler};
use runtime::lib::NativeFn;
pub use runtime::{core::Value, error::RuntimeError, interpreter::Runtime};
pub use syntax::SyntaxError;

#[macro_use]
mod macros;
mod common;
mod compiler;
mod runtime;
mod syntax;

#[derive(Default)]
pub struct Dice {
    runtime: Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: &str) -> Result<Value, DiceError> {
        let bytecode = Compiler::compile_str(input, CompilationKind::Script)?;
        self.runtime.run_bytecode(bytecode).map_err(From::from)
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

impl DiceError {
    pub fn span(&self) -> Span {
        match self {
            DiceError::SyntaxError(err) => err.span(),
            _ => todo!(),
        }
    }
}
