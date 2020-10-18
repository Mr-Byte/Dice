use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct TypeId(Uuid);

impl Default for TypeId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:#16X}", self.0.to_simple())
    }
}
