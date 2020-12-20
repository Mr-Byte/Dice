use std::fmt::{Debug, Display};

use super::FnScript;
use crate::upvalue::Upvalue;
use std::rc::Rc;

impl Debug for FnClosureInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure{{{}}}", self.fn_script)
    }
}

#[derive(Clone)]
pub struct FnClosure {
    inner: Rc<FnClosureInner>,
}

impl FnClosure {
    pub fn new(fn_script: FnScript, upvalues: Box<[Upvalue]>) -> Self {
        Self {
            inner: Rc::new(FnClosureInner { fn_script, upvalues }),
        }
    }

    pub fn fn_script(&self) -> &FnScript {
        &self.inner.fn_script
    }

    pub fn upvalues(&self) -> &[Upvalue] {
        &*self.inner.upvalues
    }
}

impl Debug for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl PartialEq for FnClosure {
    fn eq(&self, other: &Self) -> bool {
        self.inner.fn_script == other.inner.fn_script && std::ptr::eq(self.inner.upvalues.as_ptr(), other.inner.upvalues.as_ptr())
    }
}

impl Display for FnClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure{{{}}}", self.inner.fn_script)
    }
}

struct FnClosureInner {
    fn_script: FnScript,
    upvalues: Box<[Upvalue]>,
}
