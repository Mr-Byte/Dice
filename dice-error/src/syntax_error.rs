use crate::span::{Span, SpannedError};
use std::num::{ParseFloatError, ParseIntError};

#[derive(thiserror::Error, Debug, Clone)]
pub enum LexerError {
    #[error("String is not terminated with a double quote.")]
    UnterminatedString,

    #[error("Unrecognized escape sequence '{0}' found.")]
    UnrecognizedEscapeSequence(String),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum SyntaxError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String, Span),

    #[error("Function {0} has too many arguments (max 255).")]
    FnTooManyArguments(String, Span),

    #[error("Constructor has too many arguments (max 255).")]
    ConstructorTooManyArguments(Span),

    #[error("Anonymous function has too many arguments (max 255).")]
    AnonymousFnTooManyArguments(Span),

    #[error("{0}")]
    LexerError(#[source] LexerError, Span),
}

impl SpannedError for SyntaxError {
    fn span(&self) -> Span {
        match self {
            SyntaxError::UnexpectedToken(_, span) => *span,
            SyntaxError::FnTooManyArguments(_, span) => *span,
            SyntaxError::ConstructorTooManyArguments(span) => *span,
            SyntaxError::AnonymousFnTooManyArguments(span) => *span,
            SyntaxError::LexerError(_, span) => *span,
        }
    }
}
