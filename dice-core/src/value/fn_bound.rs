use gc_arena::{Collect, Gc, Mutation};

use crate::value::Value;

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct FnBound<'gc> {
    inner: Gc<'gc, FnBoundInner<'gc>>,
}

impl<'gc> FnBound<'gc> {
    pub fn new(mutation: &Mutation<'gc>, receiver: Value<'gc>, function: Value<'gc>) -> Self {
        Self {
            inner: Gc::new(mutation, FnBoundInner { receiver, function }),
        }
    }

    pub fn receiver(&self) -> Value<'gc> {
        self.inner.receiver.clone()
    }

    pub fn function(&self) -> Value<'gc> {
        self.inner.function.clone()
    }
}

impl PartialEq for FnBound<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.receiver == other.inner.receiver && self.inner.function == other.inner.function
    }
}

// impl Display for FnBound<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "FnBound{{{}}}", self.inner.function)
//     }
// }

#[derive(Collect)]
#[collect(no_drop)]
struct FnBoundInner<'gc> {
    receiver: Value<'gc>,
    function: Value<'gc>,
}

// impl Debug for FnBoundInner<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         std::fmt::Debug::fmt(&self.function, f)
//     }
// }
