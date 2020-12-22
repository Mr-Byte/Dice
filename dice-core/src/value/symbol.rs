use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    hash::Hash,
};
use string_interner::{DefaultSymbol, StringInterner};

// NOTE: Use a naive string interning system for now.
thread_local! {
    static INTERNED_SYMBOLS: RefCell<StringInterner> = RefCell::new(StringInterner::default());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol {
    inner: DefaultSymbol,
}

impl Symbol {
    pub fn as_string(self) -> String {
        INTERNED_SYMBOLS.with(|strings| {
            strings
                .borrow()
                .resolve(self.inner)
                .expect("Unable to find interned symbol.")
                .to_owned()
        })
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        INTERNED_SYMBOLS.with(|strings| Self {
            inner: strings.borrow_mut().get_or_intern(value),
        })
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<'a> From<&'a str> for Symbol {
    fn from(value: &'a str) -> Self {
        let value = value.to_string();

        INTERNED_SYMBOLS.with(|strings| Self {
            inner: strings.borrow_mut().get_or_intern(value),
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

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
