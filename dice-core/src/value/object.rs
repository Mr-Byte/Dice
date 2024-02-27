use gc_arena::{lock::RefLock, Collect, Gc, Mutation};

use crate::{
    runtime::RuntimeContext,
    type_id::TypeId,
    value::{Class, Symbol, Value, ValueMap},
};
use std::cell::Ref;

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct Object<'gc> {
    inner: Gc<'gc, ObjectInner<'gc>>,
}

impl<'gc> Object<'gc> {
    pub fn new<S, N>(ctx: &RuntimeContext<'gc, S>, class: N) -> Self
    where
        N: Into<Option<Class<'gc>>>,
    {
        Self {
            inner: Gc::new(
                ctx.mutation,
                ObjectInner {
                    class: class.into(),
                    fields: Gc::new(ctx.mutation, RefLock::new(ValueMap::default())),
                },
            ),
        }
    }

    pub fn deep_clone<S>(&self, ctx: &RuntimeContext<'gc, S>) -> Self {
        Self {
            inner: Gc::new(
                ctx.mutation,
                ObjectInner {
                    class: self.inner.class.clone(),
                    fields: self.inner.fields.clone(),
                },
            ),
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

    pub fn class(&self) -> Option<Class<'gc>> {
        self.inner.class.clone()
    }

    pub fn set_field(&self, mutation: &Mutation<'gc>, field_name: impl Into<Symbol>, value: impl Into<Value<'gc>>) {
        self.inner
            .fields
            .borrow_mut(mutation)
            .insert(field_name.into(), value.into());
    }

    pub fn field(&self, field_name: Symbol) -> Option<Value<'gc>> {
        self.inner.fields.borrow().get(&field_name).cloned()
    }

    pub fn fields(&self) -> Ref<'_, ValueMap<'gc>> {
        self.inner.fields.borrow()
    }
}

impl PartialEq for Object<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            &*self.inner as *const ObjectInner as *const u8,
            &*other.inner as *const ObjectInner as *const u8,
        )
    }
}

// impl Display for Object<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Object")?;

//         if let Some(ref class) = self.inner.class {
//             write!(f, "<{}>", class.name())?;
//         }

//         write!(f, " {{ ")?;
//         for (name, field) in self.inner.fields.borrow().iter() {
//             write!(f, "{}: {}, ", name, field)?;
//         }
//         write!(f, "}}")?;

//         Ok(())
//     }
// }

#[derive(Collect)]
#[collect(no_drop)]
struct ObjectInner<'gc> {
    class: Option<Class<'gc>>,
    fields: Gc<'gc, RefLock<ValueMap<'gc>>>,
}
