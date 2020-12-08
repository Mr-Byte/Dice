use dice_core::span::Span;

#[derive(Clone, Debug, thiserror::Error)]
#[error("{0}")]
pub struct SyntaxError(String, Span);

impl SyntaxError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self(message.into(), span)
    }
}
