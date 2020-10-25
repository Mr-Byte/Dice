use crate::{
    type_id::TypeId,
    value::{symbol::Symbol, Object, Value, ValueMap},
};
use std::{
    cell::RefCell,
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

    pub fn derive(&self, name: &str) -> Self {
        Self::with_base(name.into(), self.clone())
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

    pub fn set_method(&self, name: impl Into<Symbol>, method: impl Into<Value>) {
        // TODO: Assert method.is_function()

        self.methods.borrow_mut().insert(name.into(), method.into());
    }

    pub fn methods(&self) -> Vec<(Symbol, Value)> {
        // TODO: Make this handle multiple, conflicting methods when traits are added.
        self.methods
            .borrow()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .chain(self.base.iter().flat_map(|base| base.methods()))
            .collect::<Vec<_>>()
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
