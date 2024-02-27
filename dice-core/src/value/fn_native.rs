use gc_arena::Collect;

use crate::{error::Error, runtime::Runtime, value::Value};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub type NativeFn<'gc> = Box<dyn Fn(&mut dyn Runtime<'gc>, &[Value]) -> Result<Value<'gc>, Error>>;

#[derive(Clone, Collect)]
#[collect(require_static)]
pub struct FnNative<'gc> {
    inner: Rc<NativeFn<'gc>>,
}

impl<'gc> FnNative<'gc> {
    pub fn new(native_fn: NativeFn<'gc>) -> Self {
        Self {
            inner: Rc::new(native_fn),
        }
    }

    #[inline]
    pub fn call(&self, runtime: &mut dyn Runtime<'gc>, args: &[Value]) -> Result<Value<'gc>, Error> {
        (*self.inner)(runtime, args)
    }
}

impl Display for FnNative<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native_fn")
    }
}

impl Debug for FnNative<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native_fn")
    }
}
