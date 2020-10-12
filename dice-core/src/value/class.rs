use crate::id::type_id::TypeId;
use crate::value::{Object, Value};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class{{{}}}", self.name)
    }
}

#[derive(Debug)]
pub struct ClassInner {
    path: String,
    name: String,
    methods: RefCell<HashMap<String, Value>>,
    constructor: Option<Value>,
    object: Object,
    instance_type_id: TypeId,
}

impl ClassInner {
    pub fn path(&self) -> &str {
        &*self.path
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn methods_mut(&self) -> RefMut<'_, HashMap<String, Value>> {
        self.methods.borrow_mut()
    }

    pub fn constructor(&self) -> Option<Value> {
        self.constructor.clone()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.instance_type_id
    }
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
