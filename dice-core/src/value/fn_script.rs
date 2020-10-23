use crate::bytecode::Bytecode;
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct FnScriptInner {
    pub name: String,
    pub bytecode: Bytecode,
    id: uuid::Uuid,
}

#[derive(Clone, Debug)]
pub struct FnScript {
    inner: Rc<FnScriptInner>,
}

impl FnScript {
    pub fn new(name: String, bytecode: Bytecode, id: uuid::Uuid) -> Self {
        Self {
            inner: Rc::new(FnScriptInner { bytecode, name, id }),
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
        self.id == other.id
    }
}

impl Display for FnScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
