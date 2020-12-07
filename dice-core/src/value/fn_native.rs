use crate::{runtime::Runtime, value::Value};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub type NativeFn = Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, ()>>;

#[derive(Clone)]
pub struct FnNative(NativeFn);

impl FnNative {
    pub fn new(native_fn: NativeFn) -> Self {
        Self(native_fn)
    }

    #[inline]
    pub fn call(&self, runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, ()> {
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
