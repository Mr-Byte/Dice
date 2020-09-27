use dice_core::runtime::NativeError;

#[derive(thiserror::Error, Debug)]
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
    VariableNotFound(String),

    #[error("Unknown instruction: {0:#04X}")]
    UnknownInstruction(u8),

    #[error(transparent)]
    NativeError(#[from] NativeError),
}
