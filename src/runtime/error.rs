use crate::common::symbol::Symbol;

#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted.")]
    Aborted(String),
    #[error("Invalid global name type.")]
    InvalidGlobalNameType,
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: The target type is not a function.")]
    NotAFunction,
    #[error("Runtime Error: Variable {0} not found.")]
    VariableNotFound(Symbol),

    #[error("Unknown instruction: {0:2X}")]
    UnknownInstruction(u8),
}
