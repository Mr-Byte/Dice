use crate::{
    type_id::TypeId,
    value::{symbol::Symbol, Object, Value, ValueKind, ValueMap},
};
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::{
    any::Any,
    cell::RefCell,
    fmt::{Display, Formatter},
    ops::Deref,
    rc::Rc,
};
use wyhash::WyHash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class {
    inner: Rc<ClassInner>,
}

impl Class {
    pub fn new(name: Symbol) -> Self {
        let instance_type_id = TypeId::new();
        let mut type_ids: HashSet<_, _> = Default::default();
        type_ids.insert(instance_type_id);

        let inner = ClassInner {
            instance_type_id,
            type_ids,
            methods: Default::default(),
            object: Object::new(None),
            name,
            base: None,
        };

        Self { inner: inner.into() }
    }

    pub fn with_base(name: Symbol, base: Class) -> Self {
        let instance_type_id = TypeId::new();
        let mut type_ids: HashSet<_, _> = base.inner.type_ids.clone();
        type_ids.insert(instance_type_id);

        let inner = ClassInner {
            instance_type_id,
            type_ids,
            name,
            methods: base.inner.methods.clone(),
            object: base.inner.object.deep_clone(),
            base: Some(base),
        };

        Self { inner: inner.into() }
    }

    pub fn derive(&self, name: impl Into<Symbol>) -> Self {
        Self::with_base(name.into(), self.clone())
    }

    pub fn is_class(&self, class: &Class) -> bool {
        let type_id = class.instance_type_id();

        self.inner.type_ids.contains(&type_id)
    }

    pub fn name(&self) -> Symbol {
        self.inner.name.clone()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.inner.instance_type_id
    }

    pub fn method(&self, name: impl Into<Symbol>) -> Option<Value> {
        let name = name.into();
        self.inner.methods.borrow().get(&name).cloned()
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
    methods: RefCell<ValueMap>,
    object: Object,
    instance_type_id: TypeId,
    type_ids: HashSet<TypeId, BuildHasherDefault<WyHash>>,
    base: Option<Class>,
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
