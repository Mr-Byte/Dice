mod array;
mod class;
mod fn_bound;
mod fn_closure;
mod fn_native;
mod fn_script;
mod object;
mod string;
mod symbol;

use ahash::AHasher;
use gc_arena_derive::Collect;
use std::{collections::HashMap, hash::BuildHasherDefault};

use crate::{
    error::{
        codes::{
            INVALID_ARRAY_CONVERSION, INVALID_BOOL_CONVERSION, INVALID_CLASS_CONVERSION, INVALID_FLOAT_CONVERSION,
            INVALID_INT_CONVERSION, INVALID_OBJECT_CONVERSION, INVALID_STRING_CONVERSION, INVALID_SYMBOL_CONVERSION,
        },
        Error,
    },
    runtime::RuntimeContext,
};
pub use array::*;
pub use class::*;
pub use fn_bound::*;
pub use fn_closure::*;
pub use fn_native::*;
pub use fn_script::*;
pub use object::*;
pub use string::*;
pub use symbol::*;

pub type ValueMap<'gc> = HashMap<Symbol, Value<'gc>, BuildHasherDefault<AHasher>>;

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    FnScript(FnScript),
    FnClosure(FnClosure<'gc>),
    FnNative(FnNative<'static>),
    FnBound(FnBound<'gc>),
    Array(Array<'gc>),
    String(String),
    Symbol(Symbol),
    Object(Object<'gc>),
    Class(Class<'gc>),
}

impl<'gc> Value<'gc> {
    pub fn with_string(string: impl Into<String>) -> Self {
        Self::String(string.into())
    }

    pub fn with_symbol(string: impl Into<Symbol>) -> Self {
        Self::Symbol(string.into())
    }

    pub fn with_native_fn(native_fn: impl Into<NativeFn<'static>>) -> Self {
        Self::FnNative(FnNative::new(native_fn.into()))
    }

    pub fn with_vec<S>(ctx: &RuntimeContext<'gc, S>, vec: Vec<Value<'gc>>) -> Self {
        Value::Array(Array::from_vec(ctx, vec))
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        match self {
            Value::Bool(bool) => Ok(*bool),
            _ => Err(Error::new(INVALID_BOOL_CONVERSION)),
        }
    }

    pub fn as_int(&self) -> Result<i64, Error> {
        match self {
            Value::Int(int) => Ok(*int),
            _ => Err(Error::new(INVALID_INT_CONVERSION)),
        }
    }

    pub fn as_float(&self) -> Result<f64, Error> {
        match self {
            Value::Float(float) => Ok(*float),
            _ => Err(Error::new(INVALID_FLOAT_CONVERSION)),
        }
    }

    pub fn as_array(&self) -> Result<&Array<'gc>, Error> {
        match self {
            Value::Array(list) => Ok(list),
            _ => Err(Error::new(INVALID_ARRAY_CONVERSION)),
        }
    }

    pub fn as_string(&self) -> Result<&String, Error> {
        match self {
            Value::String(string) => Ok(string),
            _ => Err(Error::new(INVALID_STRING_CONVERSION)),
        }
    }

    pub fn as_symbol(&self) -> Result<Symbol, Error> {
        match self {
            Value::Symbol(symbol) => Ok(symbol.clone()),
            // Value::String(string) => Ok(string.into()),
            _ => Err(Error::new(INVALID_SYMBOL_CONVERSION)),
        }
    }

    pub fn as_object(&self) -> Result<&Object<'gc>, Error> {
        match self {
            Value::Object(object) => Ok(object),
            Value::Class(class) => Ok(&(**class)),
            Value::Array(array) => Ok(&(**array)),
            _ => Err(Error::new(INVALID_OBJECT_CONVERSION)),
        }
    }

    pub fn as_class(&self) -> Result<Class<'gc>, Error> {
        match self {
            Value::Class(class) => Ok(class.clone()),
            _ => Err(Error::new(INVALID_CLASS_CONVERSION)),
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

impl<'gc> Default for Value<'gc> {
    fn default() -> Self {
        Value::Null
    }
}

impl<'gc> PartialEq for Value<'gc> {
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

impl<'gc> From<NativeFn<'static>> for Value<'gc> {
    fn from(value: NativeFn<'static>) -> Self {
        Value::with_native_fn(value)
    }
}

// impl<'gc> Display for Value<'gc> {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Value::Null => write!(fmt, "null"),
//             Value::Unit => write!(fmt, "Unit"),
//             Value::Bool(bool) => bool.fmt(fmt),
//             Value::Int(int) => int.fmt(fmt),
//             Value::Float(float) => float.fmt(fmt),
//             Value::FnClosure(func) => func.fmt(fmt),
//             Value::FnScript(func) => func.fmt(fmt),
//             Value::FnNative(func) => func.fmt(fmt),
//             Value::FnBound(func) => func.fmt(fmt),
//             Value::Array(list) => list.fmt(fmt),
//             Value::String(string) => string.fmt(fmt),
//             Value::Symbol(string) => string.fmt(fmt),
//             Value::Object(object) => object.fmt(fmt),
//             Value::Class(class) => class.fmt(fmt),
//         }
//     }
// }

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Collect)]
#[collect(require_static)]
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
