use crate::{
    type_id::TypeId,
    value::{Class, ClassInner, Symbol, Value, ValueMap},
};
use std::{
    cell::{Ref, RefCell},
    fmt::{Display, Formatter},
    ops::Deref,
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

#[derive(Default, Debug)]
pub struct ObjectInner {
    class: Option<Class>,
    fields: RefCell<ValueMap>,
}

impl ObjectInner {
    pub fn type_id(&self) -> TypeId {
        self.class
            .as_ref()
            .map_or_else(TypeId::default, |class| class.instance_type_id())
    }

    pub fn is_instance_of(&self, class: &Class) -> bool {
        self.class
            .as_ref()
            .map_or_else(|| false, |self_class| self_class.is_class(class))
    }

    pub fn name(&self) -> Option<Symbol> {
        self.class.as_ref().map(Class::name)
    }

    pub fn class(&self) -> Option<Class> {
        self.class.clone()
    }

    pub fn set_field(&self, field_name: impl Into<Symbol>, value: impl Into<Value>) {
        self.fields.borrow_mut().insert(field_name.into(), value.into());
    }

    pub fn field(&self, field_name: Symbol) -> Option<Value> {
        self.fields.borrow().get(&field_name).cloned()
    }

    pub fn fields(&self) -> Ref<'_, ValueMap> {
        self.fields.borrow()
    }
}
