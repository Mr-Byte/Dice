use crate::bytecode::Bytecode;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Class {
    inner: Rc<ClassInner>,
}

#[derive(Debug)]
pub struct ClassInner {
    pub path: Option<String>,
    pub name: String,
    pub methods: HashMap<String, Bytecode>,
    pub constructor: Option<Bytecode>,
}

impl Deref for Class {
    type Target = ClassInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
