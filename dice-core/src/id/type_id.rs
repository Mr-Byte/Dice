use crate::hash::Hash;
use crate::value::Value;
use std::fmt::{Debug, Formatter};
use std::mem::Discriminant;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct TypeId(Hash);

impl TypeId {
    pub fn new<'a>(base_id: impl Into<Option<Discriminant<Value>>>, name: impl Into<Option<&'a str>>) -> Self {
        let hash = Hash::of((base_id.into(), name.into()));

        TypeId(hash)
    }
}

impl From<u64> for TypeId {
    fn from(value: u64) -> Self {
        TypeId(value.into())
    }
}

impl Into<u64> for TypeId {
    fn into(self) -> u64 {
        self.0.into()
    }
}

impl std::fmt::UpperHex for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value: u64 = self.0.into();
        std::fmt::UpperHex::fmt(&value, f)
    }
}
