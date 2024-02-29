use gc_arena::Collect;
use std::hash::Hash;
use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

#[derive(Default)]
pub struct SymbolInterner {
    interner: StringInterner<DefaultBackend>,
}

impl SymbolInterner {
    pub fn get_or_intern(&mut self, value: impl AsRef<str>) -> Symbol {
        Symbol {
            inner: self.interner.get_or_intern(value),
        }
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.interner.resolve(symbol.inner)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Collect)]
#[collect(require_static)]
#[repr(transparent)]
pub struct Symbol {
    inner: DefaultSymbol,
}
