use crate::{
    type_id::TypeId,
    value::{Class, Symbol, Value, ValueMap},
};
use std::{
    cell::{Ref, RefCell},
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Default, Clone, Debug)]
pub struct Object {
    inner: Rc<ObjectInner>,
}

impl Object {
    pub fn new<N>(class: N) -> Self
    where
        N: Into<Option<Class>>,
    {
        Self {
            inner: Rc::new(ObjectInner {
                class: class.into(),
                fields: Default::default(),
            }),
        }
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            inner: Rc::new(ObjectInner {
                class: self.inner.class.clone(),
                fields: self.inner.fields.clone(),
            }),
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.inner
            .class
            .as_ref()
            .map_or_else(TypeId::default, |class| class.instance_type_id())
    }

    pub fn is_instance_of(&self, class: &Class) -> bool {
        self.inner
            .class
            .as_ref()
            .map_or_else(|| false, |self_class| self_class.is_class(class))
    }

    pub fn name(&self) -> Option<Symbol> {
        self.inner.class.as_ref().map(Class::name)
    }

    pub fn class(&self) -> Option<Class> {
        self.inner.class.clone()
    }

    pub fn set_field(&self, field_name: impl Into<Symbol>, value: impl Into<Value>) {
        self.inner.fields.borrow_mut().insert(field_name.into(), value.into());
    }

    pub fn field(&self, field_name: Symbol) -> Option<Value> {
        self.inner.fields.borrow().get(&field_name).cloned()
    }

    pub fn fields(&self) -> Ref<'_, ValueMap> {
        self.inner.fields.borrow()
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

        if let Some(ref class) = self.inner.class {
            write!(f, "<{}>", class.name())?;
        }

        write!(f, " {{ ")?;
        for (name, field) in self.inner.fields.borrow().iter() {
            write!(f, "{}: {}, ", name, field)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

#[derive(Default, Debug)]
struct ObjectInner {
    class: Option<Class>,
    fields: RefCell<ValueMap>,
}
