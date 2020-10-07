use crate::value::Value;
use std::hash;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use twox_hash::XxHash64;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct Hash(u64);

impl Hash {
    pub fn of(item: impl hash::Hash) -> Self {
        let mut hasher = Self::hasher();
        item.hash(&mut hasher);

        Self(hasher.finish())
    }

    fn hasher() -> impl Hasher {
        BuildHasherDefault::<XxHash64>::default().build_hasher()
    }
}

impl From<u64> for Hash {
    fn from(value: u64) -> Self {
        Hash(value)
    }
}

impl Into<u64> for Hash {
    fn into(self) -> u64 {
        self.0
    }
}
