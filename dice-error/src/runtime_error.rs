use crate::{compiler_error::CompilerError, type_error::TypeError};

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted. Reason: {0}")]
    Aborted(String),
    #[error("Invalid global name type.")]
    InvalidGlobalNameType, // TODO: Move to TypeError.
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: The target type is not a function.")]
    NotAFunction, // TODO: Move to TypeError.
    #[error("Runtime Error: Variable {0} not found.")]
    VariableNotFound(String),
    #[error("Unknown instruction: {0:#04X}")]
    UnknownInstruction(u8),

    #[error(transparent)]
    TypeError(#[from] TypeError),
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    #[error(transparent)]
    FileError(#[from] std::io::Error),
}
