use crate::value::Value;
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct FnBound {
    inner: Rc<FnBoundInner>,
}

impl FnBound {
    pub fn new(receiver: Value, function: Value) -> Self {
        Self {
            inner: Rc::new(FnBoundInner { receiver, function }),
        }
    }

    pub fn receiver(&self) -> Value {
        self.inner.receiver.clone()
    }

    pub fn function(&self) -> Value {
        self.inner.function.clone()
    }
}

impl PartialEq for FnBound {
    fn eq(&self, other: &Self) -> bool {
        self.inner.receiver == other.inner.receiver && self.inner.function == other.inner.function
    }
}

impl Display for FnBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnBound{{{}}}", self.inner.function)
    }
}

struct FnBoundInner {
    receiver: Value,
    function: Value,
}

impl Debug for FnBoundInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.function, f)
    }
}
