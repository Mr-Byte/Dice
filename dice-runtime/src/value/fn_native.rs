use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use gc_arena::Collect;

use dice_core::error::Error;

use crate::value::Value;

// pub type NativeFn<'gc> = Box<dyn Fn(&mut dyn Runtime<'gc>, &[Value]) -> Result<Value<'gc>, Error>>;
pub type NativeFn<'gc> = Box<dyn Fn(&[Value]) -> Result<Value<'gc>, Error>>;

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
    pub fn call(&self, args: &[Value]) -> Result<Value<'gc>, Error> {
        (*self.inner)(args)
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
