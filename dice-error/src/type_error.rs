#[derive(thiserror::Error, Debug)]
pub enum TypeError {
    #[error("The specified value is not null.")]
    NotNull,
    #[error("The specified value is not unit.")]
    NotUnit,
    #[error("The specified value is not a bool.")]
    NotABool,
    #[error("The specified value is not an int.")]
    NotAnInt,
    #[error("The specified value is not a float.")]
    NotAFloat,
    #[error("The specified value is not a function.")]
    NotAFunction,
    #[error("The specified value is not a string.")]
    NotAString,
    #[error("The specified value is not a list.")]
    NotAList,
    #[error("The specified value is not a class.")]
    NotAnObject,
    #[error("The specified value is not an object.")]
    NotAClass,
}
