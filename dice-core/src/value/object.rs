use crate::{
    id::type_id::TypeId,
    value::{Class, Value, ValueMap},
};
use gc::{Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Object {
    inner: Gc<ObjectInner>,
}

impl Object {
    pub fn new<N>(type_id: TypeId, class: N) -> Self
    where
        N: Into<Option<Value>>,
    {
        Self {
            inner: Gc::new(ObjectInner {
                class: class.into(),
                fields: Default::default(),
                mixin_type_ids: Vec::new(),
                type_id,
            }),
        }
    }
}

impl Deref for Object {
    type Target = ObjectInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
            && std::ptr::eq(
                &*self.inner as *const ObjectInner as *const u8,
                &*other.inner as *const ObjectInner as *const u8,
            )
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object")?;

        let class = self.class.as_ref().and_then(|value| value.as_class().ok());
        if let Some(class) = class {
            write!(f, "{{{}}}", class.name())?;
        }

        Ok(())
    }
}

#[derive(Debug, Trace, Finalize)]
pub struct ObjectInner {
    class: Option<Value>,
    fields: GcCell<ValueMap>,
    #[unsafe_ignore_trace]
    type_id: TypeId,
    #[unsafe_ignore_trace]
    mixin_type_ids: Vec<TypeId>,
}

impl ObjectInner {
    pub fn fields(&self) -> GcCellRef<'_, ValueMap> {
        self.fields.borrow()
    }

    pub fn fields_mut(&self) -> GcCellRefMut<'_, ValueMap> {
        self.fields.borrow_mut()
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn name(&self) -> Option<&str> {
        self.class
            .as_ref()
            .and_then(|value| value.as_class().map(|class| class.name()).ok())
    }

    pub fn class(&self) -> Option<&Class> {
        self.class.as_ref().and_then(|value| value.as_class().ok())
    }
}
