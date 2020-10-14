use crate::span::{Span, SpannedError};

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String, Span),

    #[error("Function {0} has too many arguments (max 255).")]
    FnTooManyArguments(String, Span),

    #[error("Constructor has too many arguments (max 255).")]
    ConstructorTooManyArguments(Span),

    #[error("Anonymous function has too many arguments (max 255).")]
    AnonymousFnTooManyArguments(Span),
}

impl SpannedError for SyntaxError {
    fn span(&self) -> Span {
        match self {
            SyntaxError::UnexpectedToken(_, span) => *span,
            SyntaxError::FnTooManyArguments(_, span) => *span,
            SyntaxError::ConstructorTooManyArguments(span) => *span,
            SyntaxError::AnonymousFnTooManyArguments(span) => *span,
        }
    }
}
