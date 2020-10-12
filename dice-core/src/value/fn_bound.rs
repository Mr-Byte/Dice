use crate::value::Value;
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct FnBoundInner {
    pub receiver: Value,
    pub function: Value,
}

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
