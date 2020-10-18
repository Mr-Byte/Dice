use gc::{Finalize, Gc, Trace};
use std::any::Any;

pub trait GcAny: Any + Trace + Finalize + std::fmt::Debug {}

impl<T> GcAny for T where T: Any + Trace + Finalize + std::fmt::Debug {}

#[derive(Debug, Clone, Trace, Finalize)]
#[repr(transparent)]
pub struct GcAnyBox(Gc<Box<dyn GcAny>>);

impl GcAnyBox {
    pub fn new<T>(inner: T) -> Self
    where
        T: GcAny,
    {
        let value: Gc<Box<dyn GcAny>> = Gc::new(Box::new(inner));
        Self(value)
    }

    pub fn downcast<T>(&self) -> Option<&T>
    where
        T: GcAny,
    {
        let any = self.0.as_ref() as &dyn Any;
        any.downcast_ref()
    }
}
