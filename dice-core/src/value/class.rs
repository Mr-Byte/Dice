use crate::value::OBJECT_TYPE_ID;
use crate::{
    id::type_id::TypeId,
    value::{symbol::Symbol, Object, ValueMap},
};
use gc::{Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Class {
    inner: Gc<ClassInner>,
}

impl Class {
    pub fn new(name: Symbol, path: Symbol) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            methods: Default::default(),
            object: Object::new(OBJECT_TYPE_ID.with(Clone::clone), None),
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

#[derive(Debug, Trace, Finalize)]
pub struct ClassInner {
    path: Symbol,
    name: Symbol,
    methods: GcCell<ValueMap>,
    object: Object,
    #[unsafe_ignore_trace]
    instance_type_id: TypeId,
}

impl ClassInner {
    pub fn path(&self) -> &str {
        &*self.path
    }

    pub fn name(&self) -> &str {
        &*self.name
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
