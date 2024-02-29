use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct TypeId(Uuid);

impl TypeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:#16X}", self.0.to_u128_le())
    }
}
