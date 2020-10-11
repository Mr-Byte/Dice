use crate::bytecode::Bytecode;
use crate::id::type_id::TypeId;
use crate::value::Object;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Class {
    inner: Rc<ClassInner>,
}

impl Class {
    pub fn new(name: String, path: String) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(None, path.as_str(), name.as_str()),
            methods: Default::default(),
            constructor: None,
            object: Object::new(TypeId::new(None, None, "ClassObject"), &name),
            name,
            path,
        };

        Self { inner: inner.into() }
    }
}

#[derive(Debug)]
pub struct ClassInner {
    pub path: String,
    pub name: String,
    pub methods: HashMap<String, Bytecode>,
    pub constructor: Option<Bytecode>,
    pub object: Object,
    pub instance_type_id: TypeId,
}

impl Deref for Class {
    type Target = ClassInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl Deref for ClassInner {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
