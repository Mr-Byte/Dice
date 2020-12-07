use dice_core::span::Span;

#[derive(Clone, Debug, thiserror::Error)]
#[error("{0}")]
pub struct SyntaxError(String, Span);

impl SyntaxError {
    pub fn message(&self) -> &str {
        &self.0
    }

    pub fn span(&self) -> Span {
        self.1
    }
}
