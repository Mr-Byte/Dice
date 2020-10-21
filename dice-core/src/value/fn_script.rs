use crate::bytecode::Bytecode;
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Trace, Finalize)]
pub struct FnScriptInner {
    pub name: String,
    pub bytecode: Bytecode,
    #[unsafe_ignore_trace]
    id: uuid::Uuid,
}
use gc::{Finalize, Gc, Trace};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct FnScript {
    inner: Gc<FnScriptInner>,
}

impl FnScript {
    pub fn new(name: String, bytecode: Bytecode, id: uuid::Uuid) -> Self {
        Self {
            inner: Gc::new(FnScriptInner { bytecode, name, id }),
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
