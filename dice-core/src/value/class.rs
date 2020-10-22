use crate::{
    id::type_id::TypeId,
    value::{symbol::Symbol, Object, ValueMap},
};
use gc::{Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Debug, Trace, Finalize, Eq, PartialEq)]
pub struct Class {
    inner: Gc<ClassInner>,
}

impl Class {
    pub fn new(name: Symbol, base: Option<Class>) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            methods: Default::default(),
            object: Object::new(TypeId::default(), None),
            base,
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

#[derive(Debug, Trace, Finalize)]
pub struct ClassInner {
    name: Symbol,
    methods: GcCell<ValueMap>,
    object: Object,
    base: Option<Class>,
    instance_type_id: TypeId,
}

impl ClassInner {
    pub fn name(&self) -> Symbol {
        self.name.clone()
    }

    pub fn methods(&self) -> GcCellRef<'_, ValueMap> {
        self.methods.borrow()
    }

    pub fn methods_mut(&self) -> GcCellRefMut<'_, ValueMap> {
        self.methods.borrow_mut()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.instance_type_id
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
