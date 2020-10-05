use crate::value::{NativeFn, Value};
use dice_syntax::{Span, SpannedError};
use std::fmt::{Debug, Display};

pub trait Runtime {
    fn register_native_fn(&mut self, name: &str, native_fn: NativeFn);
    fn call_fn(&mut self, target: Value, args: &[Value]) -> Result<Value, NativeError>;
}

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
