use crate::gc_any::{GcAny, GcAnyBox};
use crate::{
    gc_any,
    id::type_id::TypeId,
    value::{Class, Value, ValueMap},
};
use gc::{Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Default, Clone, Debug, Trace, Finalize)]
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
                native_tag: GcCell::new(None),
                fields: Default::default(),
                mixin_type_ids: Vec::new(),
                type_id,
            }),
        }
    }

    pub fn with_native_tag<N>(type_id: TypeId, class: N, native_tag: GcAnyBox) -> Self
    where
        N: Into<Option<Value>>,
    {
        Self {
            inner: Gc::new(ObjectInner {
                class: class.into(),
                native_tag: GcCell::new(Some(native_tag)),
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
            write!(f, "<{}>", class.name())?;
        }

        write!(f, " {{ ")?;
        for (name, field) in self.fields.borrow().iter() {
            write!(f, "{}: {}, ", name, field)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

#[derive(Default, Debug, Trace, Finalize)]
pub struct ObjectInner {
    class: Option<Value>,
    native_tag: GcCell<Option<GcAnyBox>>,
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

    pub fn native_tag(&self) -> Option<GcCellRef<'_, GcAnyBox>> {
        gc_any::transpose(self.native_tag.borrow())
    }

    pub fn native_tag_mut(&self) -> Option<GcCellRefMut<'_, GcAnyBox>> {
        gc_any::transpose_mut(self.native_tag.borrow_mut())
    }

    pub fn set_native_tag<T: GcAny>(&mut self, tag: Option<T>) {
        *self.native_tag.borrow_mut() = tag.map(GcAnyBox::new);
    }
}
