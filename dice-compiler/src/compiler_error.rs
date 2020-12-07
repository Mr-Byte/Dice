use dice_core::{span::Span, spanned_error::SpannedError};
use dice_syntax::SyntaxError;

#[derive(Clone, Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("{0}")]
    CompilerError(String, Span),
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
}

impl CompilerError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self::CompilerError(message.into(), span)
    }
}

impl SpannedError for CompilerError {
    fn message(&self) -> &str {
        match self {
            CompilerError::CompilerError(message, _) => message,
            CompilerError::SyntaxError(err) => err.message(),
        }
    }

    fn span(&self) -> Span {
        match self {
            CompilerError::CompilerError(_, span) => *span,
            CompilerError::SyntaxError(err) => err.span(),
        }
    }
}
