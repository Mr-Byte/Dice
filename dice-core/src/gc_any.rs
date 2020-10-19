use downcast_rs::{impl_downcast, Downcast};
use gc::{Finalize, Gc, Trace};
use std::any::Any;

pub trait GcAny: Any + Trace + Finalize + std::fmt::Debug + Downcast {}

impl_downcast!(GcAny);

impl<T> GcAny for T where T: Any + Trace + Finalize + std::fmt::Debug + Downcast {}

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
        self.0.downcast_ref()
    }
}
