use std::thread::LocalKey;

pub mod operator {
    pub static MUL: &str = "#mul";
    pub static DIV: &str = "#div";
    pub static REM: &str = "#rem";
    pub static ADD: &str = "#add";
    pub static SUB: &str = "#sub";
    pub static GT: &str = "#gt";
    pub static GTE: &str = "#gte";
    pub static LT: &str = "#lt";
    pub static LTE: &str = "#lte";
    pub static EQ: &str = "#eq";
    pub static NEQ: &str = "#neq";
    pub static RANGE_INCLUSIVE: &str = "#range_inclusive";
    pub static RANGE_EXCLUSIVE: &str = "#range_exclusive";
}

pub mod module {
    pub static EXPORT: &str = "#export";
}

pub mod class {
    pub static SELF: &str = "self";
    pub static SUPER: &str = "super";
    pub static NEW: &str = "new";
}

pub mod iterator {
    pub static NEXT: &str = "next";
    pub static VALUE: &str = "value";
    pub static DONE: &str = "is_done";
    pub static ITER: &str = "iter";
}

pub mod object {
    pub static TO_STRING: &str = "to_string";
    pub static ANY_CLASS: &str = "Any";
    pub static MODULE_CLASS: &str = "Module";
}

pub mod error {
    pub static IS_OK: &str = "is_ok";
    pub static RESULT: &str = "result";
}

pub trait ProtocolSymbol {
    fn get(&'static self) -> &str;
}

impl ProtocolSymbol for LocalKey<&str> {
    fn get(&'static self) -> &str {
        self.with(Clone::clone)
    }
}
