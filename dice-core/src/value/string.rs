use gc::{Finalize, Trace};
use std::rc::Rc;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Debug, Hash, Clone, Trace, Finalize, PartialEq, Eq)]
#[repr(transparent)]
pub struct String {
    // NOTE: Strings cannot have references to anything other than a string, so they're safe to store in an Rc and don't need to be traced.
    #[unsafe_ignore_trace]
    inner: Rc<str>,
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        Self { inner: value.into() }
    }
}

impl<'a> From<&'a std::string::String> for String {
    fn from(value: &'a std::string::String) -> Self {
        Self {
            inner: value.clone().into(),
        }
    }
}

impl<'a> From<&'a str> for String {
    fn from(value: &'a str) -> Self {
        Self { inner: value.into() }
    }
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        &*self.inner
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
