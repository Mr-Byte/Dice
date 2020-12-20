use crate::value::Symbol;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    rc::Rc,
};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct String {
    inner: Rc<str>,
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        Self { inner: value.into() }
    }
}

impl<'a> From<&'a std::string::String> for String {
    fn from(value: &'a std::string::String) -> Self {
        Self { inner: value.clone().into() }
    }
}

impl From<&'_ str> for String {
    fn from(value: &'_ str) -> Self {
        Self { inner: value.into() }
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        Self { inner: (&*value).into() }
    }
}

impl From<&'_ Symbol> for String {
    fn from(value: &Symbol) -> Self {
        Self { inner: (&**value).into() }
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
