use gc::{Finalize, Trace};
use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Display, Formatter},
    hash::{BuildHasherDefault, Hash, Hasher},
    ops::Deref,
    rc::Rc,
};
use wyhash::WyHash;

// NOTE: Use a naive string interning system for now.

thread_local! {
    static INTERNED_SYMBOLS: RefCell<HashSet<Rc<str>, BuildHasherDefault<WyHash>>> = Default::default();
}

#[derive(Debug, Clone, Trace, Finalize, Eq)]
#[repr(transparent)]
pub struct Symbol {
    #[unsafe_ignore_trace]
    inner: Rc<str>,
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        INTERNED_SYMBOLS.with(|strings| {
            if let Some(interned) = strings.borrow().get(&*value) {
                return Self {
                    inner: interned.clone(),
                };
            }

            let key: Rc<str> = value.into();
            strings.borrow_mut().insert(key.clone());

            Self { inner: key }
        })
    }
}

impl<'a> From<&'a str> for Symbol {
    fn from(value: &'a str) -> Self {
        INTERNED_SYMBOLS.with(|strings| {
            if let Some(interned) = strings.borrow().get(value) {
                return Self {
                    inner: interned.clone(),
                };
            }

            let key: Rc<str> = value.into();
            strings.borrow_mut().insert(key.clone());

            Self { inner: key }
        })
    }
}

impl From<super::String> for Symbol {
    fn from(value: super::String) -> Self {
        value.into()
    }
}

impl<'a> From<&'a super::String> for Symbol {
    fn from(value: &'a super::String) -> Self {
        value.as_ref().into()
    }
}

impl Into<super::String> for Symbol {
    fn into(self) -> super::String {
        super::String::from(&*self.inner)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &*self.inner
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
