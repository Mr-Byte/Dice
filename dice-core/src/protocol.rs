use crate::value::Symbol;
use std::thread::LocalKey;

pub mod operator {
    use super::*;

    thread_local! {
        pub static MUL: Symbol = "#mul".into();
        pub static DIV: Symbol = "#div".into();
        pub static REM: Symbol = "#rem".into();
        pub static ADD: Symbol = "#add".into();
        pub static SUB: Symbol = "#sub".into();
        pub static GT: Symbol = "#gt".into();
        pub static GTE: Symbol = "#gte".into();
        pub static LT: Symbol = "#lt".into();
        pub static LTE: Symbol = "#lte".into();
        pub static EQ: Symbol = "#eq".into();
        pub static NEQ: Symbol = "#neq".into();
        pub static RANGE_INCLUSIVE: Symbol = "#range_inclusive".into();
        pub static RANGE_EXCLUSIVE: Symbol = "#range_exclusive".into();
    }
}

pub mod module {
    use super::*;

    thread_local! {
        pub static EXPORT: Symbol = "#export".into();
    }
}

pub mod class {
    use super::*;

    thread_local! {
        pub static SELF: Symbol = "self".into();
        pub static SUPER: Symbol = "super".into();
        pub static NEW: Symbol = "new".into();
    }
}

pub mod iterator {
    use super::*;

    thread_local! {
        pub static NEXT: Symbol = "next".into();
        pub static VALUE: Symbol = "value".into();
        pub static DONE: Symbol = "is_done".into();
        pub static ITER: Symbol = "iter".into();
    }
}

pub mod object {
    use super::*;

    thread_local! {
        pub static TO_STRING: Symbol = "to_string".into();
        pub static ANY_CLASS: Symbol = "Any".into();
        pub static MODULE_CLASS: Symbol = "Module".into();
    }
}

pub mod error {
    use super::*;

    thread_local! {
        pub static IS_OK: Symbol = "is_ok".into();
        pub static RESULT: Symbol = "result".into();
    }
}

pub trait ProtocolSymbol {
    fn get(&'static self) -> Symbol;
}

impl ProtocolSymbol for LocalKey<Symbol> {
    fn get(&'static self) -> Symbol {
        self.with(Clone::clone)
    }
}

impl<S> From<&'static S> for Symbol
where
    S: ProtocolSymbol + 'static,
{
    fn from(value: &'static S) -> Self {
        value.get()
    }
}
