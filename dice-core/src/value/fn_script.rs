use super::Symbol;
use dice_bytecode::Bytecode;
use gc_arena::Collect;
use std::rc::Rc;

#[derive(Clone, Debug, Collect)]
#[collect(no_drop)]
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

// impl Display for FnScript {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.inner.name)
//     }
// }

#[derive(Debug, Collect)]
#[collect(no_drop)]
struct FnScriptInner {
    name: Symbol,
    #[collect(require_static)]
    bytecode: Bytecode,
    #[collect(require_static)]
    id: uuid::Uuid,
}
