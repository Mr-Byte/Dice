use crate::{
    gc_any,
    gc_any::{GcAny, GcAnyBox},
    id::type_id::TypeId,
    value::{Class, ClassInner, Symbol, ValueMap},
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
    pub fn new<N>(class: N) -> Self
    where
        N: Into<Option<Class>>,
    {
        Self {
            inner: Gc::new(ObjectInner {
                class: class.into(),
                native_tag: GcCell::new(None),
                fields: Default::default(),
            }),
        }
    }

    pub fn with_native_tag<N>(class: N, native_tag: GcAnyBox) -> Self
    where
        N: Into<Option<Class>>,
    {
        Self {
            inner: Gc::new(ObjectInner {
                class: class.into(),
                native_tag: GcCell::new(Some(native_tag)),
                fields: Default::default(),
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
        std::ptr::eq(
            &*self.inner as *const ObjectInner as *const u8,
            &*other.inner as *const ObjectInner as *const u8,
        )
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object")?;

        if let Some(ref class) = self.class {
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
    class: Option<Class>,
    native_tag: GcCell<Option<GcAnyBox>>,
    fields: GcCell<ValueMap>,
}

impl ObjectInner {
    pub fn fields(&self) -> GcCellRef<'_, ValueMap> {
        self.fields.borrow()
    }

    pub fn fields_mut(&self) -> GcCellRefMut<'_, ValueMap> {
        self.fields.borrow_mut()
    }

    pub fn type_id(&self) -> TypeId {
        self.class
            .as_ref()
            .map_or_else(TypeId::default, |class| class.instance_type_id())
    }

    pub fn has_type_id(&self, type_id: TypeId) -> bool {
        dbg!(&type_id);
        dbg!(self
            .class
            .as_ref()
            .map_or_else(|| false, |class| class.contains_type_id(type_id)))
    }

    pub fn name(&self) -> Option<Symbol> {
        self.class.as_deref().map(ClassInner::name)
    }

    pub fn class(&self) -> Option<Class> {
        self.class.clone()
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
