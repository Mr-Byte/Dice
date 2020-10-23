use crate::{
    id::type_id::TypeId,
    value::{symbol::Symbol, Object, Value, ValueMap},
};
use std::{
    cell::{RefCell, RefMut},
    fmt::{Display, Formatter},
    ops::Deref,
    rc::Rc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class {
    inner: Rc<ClassInner>,
}

impl Class {
    pub fn new(name: Symbol) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            base: Default::default(),
            methods: Default::default(),
            object: Object::new(None),
            name,
        };

        Self { inner: inner.into() }
    }

    pub fn with_base(name: Symbol, base: Class) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            methods: Default::default(),
            object: Object::new(None),
            base: Some(base),
            name,
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
    name: Symbol,
    base: Option<Class>,
    methods: RefCell<ValueMap>,
    object: Object,
    instance_type_id: TypeId,
}

impl ClassInner {
    pub fn name(&self) -> Symbol {
        self.name.clone()
    }

    // TODO: Replace this with add_method
    pub fn methods_mut(&self) -> RefMut<'_, ValueMap> {
        self.methods.borrow_mut()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.instance_type_id
    }

    pub fn contains_type_id(&self, type_id: TypeId) -> bool {
        // TODO: Expand this to check trait_type_ids when traits are added.
        self.instance_type_id == type_id
            || self
                .base
                .as_ref()
                .map_or_else(|| false, |base| base.contains_type_id(type_id))
    }

    pub fn method(&self, name: &Symbol) -> Option<Value> {
        self.methods
            .borrow()
            .get(&name)
            .cloned()
            .or_else(|| self.base.as_ref().and_then(|base| base.method(name)))
    }

    pub fn base(&self) -> Option<Class> {
        self.base.clone()
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

impl PartialEq for ClassInner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_id() == other.type_id()
    }
}

impl Eq for ClassInner {}
