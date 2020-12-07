use dice_core::span::Span;
use dice_core::spanned_error::SpannedError;

#[derive(Clone, Debug, thiserror::Error)]
#[error("{0}")]
pub struct SyntaxError(String, Span);

impl SpannedError for SyntaxError {
    fn message(&self) -> &str {
        &self.0
    }

    fn span(&self) -> Span {
        self.1
    }
}
