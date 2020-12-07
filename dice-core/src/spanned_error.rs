use crate::span::Span;

pub trait SpannedError {
    fn message(&self) -> &str;
    fn span(&self) -> Span;
}
