use dice_core::span::Span;
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
