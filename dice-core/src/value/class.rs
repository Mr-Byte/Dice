use crate::{
    type_id::TypeId,
    value::{symbol::Symbol, Object, Value, ValueKind, ValueMap},
};
use std::{
    any::Any,
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

    pub fn derive(&self, name: impl Into<Symbol>) -> Self {
        Self::with_base(name.into(), self.clone())
    }

    pub fn is_class(&self, class: &Class) -> bool {
        let type_id = class.instance_type_id();

        self.instance_type_id() == type_id
            || self
                .inner
                .base
                .as_ref()
                .map_or_else(|| false, |base| base.is_class(class))
    }

    pub fn name(&self) -> Symbol {
        self.inner.name.clone()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.inner.instance_type_id
    }

    pub fn method(&self, name: impl Into<Symbol>) -> Option<Value> {
        let name = name.into();
        self.inner
            .methods
            .borrow()
            .get(&name)
            .cloned()
            .or_else(|| self.inner.base.as_ref().and_then(|base| base.method(name)))
    }

    pub fn set_method(&self, name: impl Into<Symbol>, method: impl Into<Value>) {
        let method = method.into();

        if method.kind() != ValueKind::Function {
            // TODO: Return error.
        }

        self.inner.methods.borrow_mut().insert(name.into(), method);
    }

    pub fn methods(&self) -> Vec<(Symbol, Value)> {
        // TODO: Make this handle multiple, conflicting methods when traits are added.
        self.inner
            .methods
            .borrow()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .chain(self.inner.base.iter().flat_map(|base| base.methods()))
            .collect::<Vec<_>>()
    }

    pub fn base(&self) -> Option<Class> {
        self.inner.base.clone()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class{{{}}}", self.inner.name)
    }
}

#[derive(Debug)]
struct ClassInner {
    name: Symbol,
    base: Option<Class>,
    methods: RefCell<ValueMap>,
    object: Object,
    instance_type_id: TypeId,
}

impl Deref for Class {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.inner.object
    }
}

impl PartialEq for ClassInner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_id() == other.type_id()
    }
}

impl Eq for ClassInner {}
