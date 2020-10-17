use gc::{Finalize, Trace};
use std::collections::HashSet;
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
    static INTERNED_STRINGS: RefCell<HashSet<Rc<str>, BuildHasherDefault<WyHash>>> = Default::default();
}

#[derive(Debug, Clone, Trace, Finalize, Eq)]
#[repr(transparent)]
pub struct DiceString {
    #[unsafe_ignore_trace]
    inner: Rc<str>,
}

impl From<String> for DiceString {
    fn from(value: String) -> Self {
        INTERNED_STRINGS.with(|strings| {
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

impl<'a> From<&'a str> for DiceString {
    fn from(value: &'a str) -> Self {
        INTERNED_STRINGS.with(|strings| {
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

impl PartialEq for DiceString {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Hash for DiceString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl Deref for DiceString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl AsRef<str> for DiceString {
    fn as_ref(&self) -> &str {
        &*self.inner
    }
}

impl Display for DiceString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
