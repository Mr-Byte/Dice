use crate::runtime::lib::{self, List};
use lib::{FnClosure, FnNative, FnScript};
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug)]
#[repr(u8)]
// TODO: How to get this down to 16-bytes?
pub enum Value {
    None(lib::None),
    Unit(lib::Unit),
    Bool(bool),
    Int(i64),
    Float(f64),
    FnClosure(FnClosure),
    FnScript(FnScript),
    FnNative(FnNative),
    List(List),
    String(Rc<String>),
}

static_assertions::assert_eq_size!([u8; 16], Value);

impl Value {
    pub const NONE: Self = Value::None(lib::None);
    pub const UNIT: Self = Value::Unit(lib::Unit);

    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None(_))
    }

    #[inline]
    pub fn is_unit(&self) -> bool {
        matches!(self, Value::Unit(_))
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    #[inline]
    pub fn is_fn(&self) -> bool {
        matches!(self, Value::FnClosure(_) | Value::FnScript(_) | Value::FnNative(_))
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    #[inline]
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::None(_), Value::None(_)) => true,
            (Value::Unit(_), Value::Unit(_)) => true,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Int(lhs), Value::Int(rhs)) => lhs == rhs,
            (Value::Float(lhs), Value::Float(rhs)) => lhs == rhs,
            (Value::FnClosure(lhs), Value::FnClosure(rhs)) => lhs == rhs,
            (Value::FnScript(lhs), Value::FnScript(rhs)) => lhs == rhs,
            (Value::List(lhs), Value::List(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::None(none) => none.fmt(fmt),
            Value::Unit(unit) => unit.fmt(fmt),
            Value::Bool(bool) => bool.fmt(fmt),
            Value::Int(int) => int.fmt(fmt),
            Value::Float(float) => float.fmt(fmt),
            Value::FnClosure(func) => func.fmt(fmt),
            Value::FnScript(func) => func.fmt(fmt),
            Value::FnNative(func) => func.fmt(fmt),
            Value::List(list) => list.fmt(fmt),
            Value::String(string) => string.fmt(fmt),
        }
    }
}
