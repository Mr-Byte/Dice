use crate::value::Value;
use gc::{Finalize, Gc, Trace};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Trace, Finalize)]
pub struct FnBoundInner {
    pub receiver: Value,
    pub function: Value,
}

#[derive(Clone, Debug, Trace, Finalize)]
pub struct FnBound {
    inner: Gc<FnBoundInner>,
}

impl FnBound {
    pub fn new(receiver: Value, function: Value) -> Self {
        Self {
            inner: Gc::new(FnBoundInner { receiver, function }),
        }
    }
}

impl Deref for FnBound {
    type Target = FnBoundInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for FnBound {
    fn eq(&self, other: &Self) -> bool {
        self.receiver == other.receiver && self.function == other.function
    }
}

impl Display for FnBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnBound{}", self.function)
    }
}
