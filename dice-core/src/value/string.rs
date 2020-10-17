use gc::{Finalize, Gc, Trace};
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[derive(Debug, Clone, Trace, Finalize, PartialEq, Eq)]
#[repr(transparent)]
pub struct DString {
    inner: Gc<String>,
}

impl From<String> for DString {
    fn from(value: String) -> Self {
        Self { inner: value.into() }
    }
}

impl<'a> From<&'a str> for DString {
    fn from(value: &'a str) -> Self {
        Self {
            inner: value.to_owned().into(),
        }
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

impl AsRef<str> for DString {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

impl Display for DString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_str())
    }
}
