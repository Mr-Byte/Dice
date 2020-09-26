use super::lexer::Token;
use crate::common::span::Span;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),

    #[error("Function {0} has too many arguments (max 255).")]
    FnTooManyArguments(String, Span),

    #[error("Anonymous function has too many arguments (max 255).")]
    AnonymousFnTooManyArguments(Span),
}
