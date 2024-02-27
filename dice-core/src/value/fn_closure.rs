use super::FnScript;
use crate::upvalue::Upvalue;
use gc_arena::{Collect, Gc, Mutation};

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct FnClosure<'gc> {
    inner: Gc<'gc, FnClosureInner<'gc>>,
}

impl<'gc> FnClosure<'gc> {
    pub fn new(mutation: &Mutation<'gc>, fn_script: FnScript, upvalues: Box<[Upvalue<'gc>]>) -> Self {
        Self {
            inner: Gc::new(mutation, FnClosureInner { fn_script, upvalues }),
        }
    }

    pub fn fn_script(&self) -> &FnScript {
        &self.inner.fn_script
    }

    pub fn upvalues(&self) -> &[Upvalue<'gc>] {
        &*self.inner.upvalues
    }
}

// impl Debug for FnClosure<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.inner)
//     }
// }

impl PartialEq for FnClosure<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.fn_script == other.inner.fn_script
            && std::ptr::eq(self.inner.upvalues.as_ptr(), other.inner.upvalues.as_ptr())
    }
}

// impl Display for FnClosure<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "closure{{{}}}", self.inner.fn_script)
//     }
// }

#[derive(Collect)]
#[collect(no_drop)]
struct FnClosureInner<'gc> {
    fn_script: FnScript,
    upvalues: Box<[Upvalue<'gc>]>,
}

// impl Debug for FnClosureInner<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Closure{{{}}}", self.fn_script)
//     }
// }
