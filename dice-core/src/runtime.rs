use dice_syntax::{Span, SpannedError};
use static_assertions::_core::fmt::Debug;
use std::fmt::Display;

pub trait Runtime {}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct NativeError {
    #[from]
    source: anyhow::Error,
}

impl NativeError {
    pub fn span<E>(&self) -> Option<Span>
    where
        E: SpannedError + Display + Debug + Send + Sync + 'static,
    {
        self.source.downcast_ref::<E>().map(|err: &E| err.span())
    }
}
