use gc::{Finalize, Gc, GcCell, Trace};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ops::Deref;
use wyhash::WyHash;

thread_local! {
    static INTERNED_STRINGS: GcCell<HashMap<String, DString, BuildHasherDefault<WyHash>>> = Default::default();
}

#[derive(Debug, Clone, Trace, Finalize, Eq)]
pub struct DString {
    inner: Gc<String>,
}

impl From<String> for DString {
    fn from(value: String) -> Self {
        INTERNED_STRINGS.with(|strings| {
            strings
                .borrow_mut()
                .entry(value.clone())
                .or_insert_with(|| DString { inner: value.into() })
                .clone()
        })
    }
}

impl<'a> From<&'a str> for DString {
    fn from(value: &'a str) -> Self {
        INTERNED_STRINGS.with(|strings| {
            strings
                .borrow_mut()
                .entry(value.to_owned())
                .or_insert_with(|| DString {
                    inner: value.to_owned().into(),
                })
                .clone()
        })
    }
}

impl PartialEq for DString {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Hash for DString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.as_str().hash(state)
    }
}

impl Deref for DString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner.as_str()
    }
}

impl Display for DString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_str())
    }
}
