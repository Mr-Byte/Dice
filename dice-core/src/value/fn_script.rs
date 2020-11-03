use crate::{bytecode::Bytecode, value::Symbol};
use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
pub struct FnScript {
    inner: Rc<FnScriptInner>,
}

impl FnScript {
    pub fn new(name: impl Into<Symbol>, bytecode: Bytecode, id: uuid::Uuid) -> Self {
        Self {
            inner: Rc::new(FnScriptInner {
                bytecode,
                name: name.into(),
                id,
            }),
        }
    }

    pub fn bytecode(&self) -> &Bytecode {
        &self.inner.bytecode
    }

    pub fn name(&self) -> Symbol {
        self.inner.name.clone()
    }
}

impl PartialEq for FnScript {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl Display for FnScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.name)
    }
}

#[derive(Debug)]
struct FnScriptInner {
    name: Symbol,
    bytecode: Bytecode,
    id: uuid::Uuid,
}
