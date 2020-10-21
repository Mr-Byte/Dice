mod array;
mod class;
mod fn_bound;
mod fn_closure;
mod fn_native;
mod fn_script;
mod object;
mod string;
mod symbol;

use dice_error::type_error::TypeError;
use gc::{Finalize, Trace};
use std::{collections::HashMap, fmt::Display, hash::BuildHasherDefault};
use wyhash::WyHash;

pub use array::*;
pub use class::*;
pub use fn_bound::*;
pub use fn_closure::*;
pub use fn_native::*;
pub use fn_script::*;
pub use object::*;
pub use string::*;
pub use symbol::*;

pub type ValueMap = HashMap<Symbol, Value, BuildHasherDefault<WyHash>>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Trace, Finalize)]
#[repr(u8)]
pub enum ValueKind {
    Null,
    Unit,
    Bool,
    Int,
    Float,
    Function,
    Array,
    String,
    Symbol,
    Object,
    Class,
}

#[derive(Clone, Debug, Trace, Finalize)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    FnClosure(FnClosure),
    FnScript(FnScript),
    FnNative(FnNative),
    FnBound(FnBound),
    Array(Array),
    String(String),
    Symbol(Symbol),
    Object(Object),
    Class(Class),
}

impl Value {
    pub fn with_string(string: impl Into<String>) -> Self {
        Self::String(string.into())
    }

    pub fn with_symbol(string: impl Into<Symbol>) -> Self {
        Self::Symbol(string.into())
    }

    pub fn with_native_fn(native_fn: impl Into<NativeFn>) -> Self {
        Self::FnNative(FnNative::new(native_fn.into()))
    }

    pub fn as_bool(&self) -> Result<bool, TypeError> {
        match self {
            Value::Bool(bool) => Ok(*bool),
            _ => Err(TypeError::NotABool),
        }
    }

    pub fn as_int(&self) -> Result<i64, TypeError> {
        match self {
            Value::Int(int) => Ok(*int),
            _ => Err(TypeError::NotAFunction),
        }
    }

    pub fn as_float(&self) -> Result<f64, TypeError> {
        match self {
            Value::Float(float) => Ok(*float),
            _ => Err(TypeError::NotAFloat),
        }
    }

    pub fn as_array(&self) -> Result<&Array, TypeError> {
        match self {
            Value::Array(list) => Ok(list),
            _ => Err(TypeError::NotAList),
        }
    }

    pub fn as_string(&self) -> Result<&String, TypeError> {
        match self {
            Value::String(string) => Ok(string),
            _ => Err(TypeError::NotAList),
        }
    }

    pub fn as_symbol(&self) -> Result<Symbol, TypeError> {
        match self {
            Value::Symbol(symbol) => Ok(symbol.clone()),
            Value::String(string) => Ok(string.into()),
            _ => Err(TypeError::NotAString),
        }
    }

    pub fn as_object(&self) -> Result<&Object, TypeError> {
        match self {
            Value::Object(object) => Ok(object),
            Value::Class(class) => Ok(&(**class)),
            _ => Err(TypeError::NotAnObject),
        }
    }

    pub fn as_class(&self) -> Result<Class, TypeError> {
        match self {
            Value::Class(class) => Ok(class.clone()),
            _ => Err(TypeError::NotAClass),
        }
    }

    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Null => ValueKind::Null,
            Value::Unit => ValueKind::Unit,
            Value::Bool(_) => ValueKind::Bool,
            Value::Int(_) => ValueKind::Int,
            Value::Float(_) => ValueKind::Float,
            Value::FnClosure(_) => ValueKind::Function,
            Value::FnScript(_) => ValueKind::Function,
            Value::FnNative(_) => ValueKind::Function,
            Value::FnBound(_) => ValueKind::Function,
            Value::Array(_) => ValueKind::Array,
            Value::String(_) => ValueKind::String,
            Value::Symbol(_) => ValueKind::Symbol,
            Value::Object(_) => ValueKind::Object,
            Value::Class(_) => ValueKind::Class,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl PartialEq for Value {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Unit, Value::Unit) => true,
            (Value::Bool(lhs), Value::Bool(rhs)) => *lhs == *rhs,
            (Value::Int(lhs), Value::Int(rhs)) => *lhs == *rhs,
            (Value::Float(lhs), Value::Float(rhs)) => *lhs == *rhs,
            (Value::FnClosure(lhs), Value::FnClosure(rhs)) => lhs == rhs,
            (Value::FnScript(lhs), Value::FnScript(rhs)) => lhs == rhs,
            (Value::Array(lhs), Value::Array(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Symbol(lhs), Value::Symbol(rhs)) => lhs == rhs,
            (Value::Object(lhs), Value::Object(rhs)) => lhs == rhs,
            (Value::Class(lhs), Value::Class(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(fmt, "null"),
            Value::Unit => write!(fmt, "Unit"),
            Value::Bool(bool) => bool.fmt(fmt),
            Value::Int(int) => int.fmt(fmt),
            Value::Float(float) => float.fmt(fmt),
            Value::FnClosure(func) => func.fmt(fmt),
            Value::FnScript(func) => func.fmt(fmt),
            Value::FnNative(func) => func.fmt(fmt),
            Value::FnBound(func) => func.fmt(fmt),
            Value::Array(list) => list.fmt(fmt),
            Value::String(string) => string.fmt(fmt),
            Value::Symbol(string) => string.fmt(fmt),
            Value::Object(object) => object.fmt(fmt),
            Value::Class(class) => class.fmt(fmt),
        }
    }
}
