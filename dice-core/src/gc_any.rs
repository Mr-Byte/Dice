use downcast_rs::{impl_downcast, Downcast};
use gc::{Finalize, Gc, GcCellRef, GcCellRefMut, Trace};
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

pub fn transpose(original: GcCellRef<Option<GcAnyBox>>) -> Option<GcCellRef<GcAnyBox>> {
    match *original {
        Some(_) => Some(GcCellRef::map(original, |value| match value {
            Some(value) => value,
            None => unreachable!(),
        })),
        None => None,
    }
}

pub fn transpose_mut(original: GcCellRefMut<Option<GcAnyBox>>) -> Option<GcCellRefMut<GcAnyBox>> {
    match *original {
        Some(_) => Some(GcCellRefMut::map(original, |value| match value {
            Some(value) => value,
            None => unreachable!(),
        })),
        None => None,
    }
}
