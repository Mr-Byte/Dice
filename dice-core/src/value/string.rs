use gc::{Finalize, Trace};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::rc::Rc;
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};
use wyhash::WyHash;

// NOTE: Use a naive string interning system for now.

thread_local! {
    static INTERNED_STRINGS: RefCell<HashMap<String, DString, BuildHasherDefault<WyHash>>> = Default::default();
}

#[derive(Debug, Clone, Trace, Finalize, Eq)]
#[repr(transparent)]
pub struct DString {
    #[unsafe_ignore_trace]
    inner: Rc<str>,
}

impl From<String> for DString {
    fn from(value: String) -> Self {
        INTERNED_STRINGS.with(|strings| {
            strings
                .borrow_mut()
                .entry(value.clone())
                .or_insert_with(|| Self { inner: value.into() })
                .clone()
        })
    }
}

impl<'a> From<&'a str> for DString {
    fn from(value: &'a str) -> Self {
        INTERNED_STRINGS.with(|strings| {
            strings
                .borrow_mut()
                .entry(value.to_owned().clone())
                .or_insert_with(|| Self { inner: value.into() })
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
        self.inner.hash(state)
    }
}

impl Deref for DString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl AsRef<str> for DString {
    fn as_ref(&self) -> &str {
        &*self.inner
    }
}

impl Display for DString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
