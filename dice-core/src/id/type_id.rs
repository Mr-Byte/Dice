use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct TypeId(Uuid);

impl TypeId {
    pub fn new() -> Self {
        TypeId(Uuid::new_v4())
    }
}

impl std::fmt::UpperHex for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0.to_simple(), f)
    }
}
