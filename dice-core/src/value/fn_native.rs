use crate::runtime::{NativeError, Runtime};
use crate::value::Value;
use std::fmt::{Debug, Display};

pub type NativeFn = fn(&mut dyn Runtime, &[Value]) -> Result<Value, NativeError>;

#[derive(Clone)]
pub struct FnNative(NativeFn);

impl FnNative {
    pub fn new(native_fn: NativeFn) -> Self {
        Self(native_fn)
    }

    #[inline]
    pub fn call(&self, runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, NativeError> {
        self.0(runtime, args)
    }
}

impl Display for FnNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native_fn")
    }
}

impl Debug for FnNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native_fn")
    }
}
