use std::{fmt::Display, ops::Deref, rc::Rc};

use crate::runtime::interpreter::bytecode::Bytecode;

#[derive(Debug)]
pub struct FnScriptInner {
    pub arity: usize,
    pub name: String,
    pub bytecode: Bytecode,
    id: uuid::Uuid,
}

#[derive(Clone, Debug)]
pub struct FnScript {
    inner: Rc<FnScriptInner>,
}

impl FnScript {
    pub fn new(name: String, arity: usize, bytecode: Bytecode, id: uuid::Uuid) -> Self {
        Self {
            inner: Rc::new(FnScriptInner {
                arity,
                bytecode,
                name,
                id,
            }),
        }
    }
}

impl Deref for FnScript {
    type Target = FnScriptInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for FnScript {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.id == other.id
    }
}

impl Display for FnScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.name, self.arity)
    }
}
