#[derive(thiserror::Error, Debug)]
pub enum TypeError {
    #[error("The specified value is not a string.")]
    NotAString,
}
