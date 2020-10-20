mod array;
mod class;
mod classes;
mod fn_bound;
mod fn_closure;
mod fn_native;
mod fn_script;
mod object;
mod string;
mod symbol;

use crate::id::type_id::TypeId;
use dice_error::type_error::TypeError;
use gc::{Finalize, Trace};
use std::cell::RefCell;
use std::{collections::HashMap, fmt::Display, hash::BuildHasherDefault};
use wyhash::WyHash;

use crate::protocol::class::NEW;
use crate::runtime::Runtime;
pub use array::*;
pub use class::*;
use dice_error::runtime_error::RuntimeError;
use downcast_rs::__alloc::rc::Rc;
pub use fn_bound::*;
pub use fn_closure::*;
pub use fn_native::*;
pub use fn_script::*;
pub use object::*;
pub use string::*;
pub use symbol::*;

pub type ValueMap = HashMap<Symbol, Value, BuildHasherDefault<WyHash>>;

thread_local! {
    pub static NULL_TYPE_ID: TypeId = TypeId::default();
    pub static UNIT_TYPE_ID: TypeId = TypeId::default();
    pub static BOOL_TYPE_ID: TypeId = TypeId::default();
    pub static FLOAT_TYPE_ID: TypeId = TypeId::default();
    pub static FN_CLOSURE_TYPE_ID: TypeId = TypeId::default();
    pub static FN_SCRIPT_TYPE_ID: TypeId = TypeId::default();
    pub static FN_NATIVE_TYPE_ID: TypeId = TypeId::default();
    pub static FN_BOUND_TYPE_ID: TypeId = TypeId::default();
    pub static ARRAY_TYPE_ID: TypeId = TypeId::default();
    pub static STRING_TYPE_ID: TypeId = TypeId::default();
    pub static SYMBOL_TYPE_ID: TypeId = TypeId::default();
    pub static OBJECT_TYPE_ID: TypeId = TypeId::default();

    static TYPE_CLASSES: RefCell<HashMap<Symbol, Class, BuildHasherDefault<WyHash>>> = Default::default();
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

    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    #[inline]
    pub fn is_unit(&self) -> bool {
        matches!(self, Value::Unit)
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
        matches!(self, Value::FnClosure(_) | Value::FnScript(_) | Value::FnNative(_) | Value::FnBound(_))
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_) | Value::Symbol(_))
    }

    #[inline]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }

    #[inline]
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    #[inline]
    pub fn is_class(&self) -> bool {
        matches!(self, Value::Class(_))
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

    pub fn class(&self) -> Option<Class> {
        match self {
            Value::Object(object) => object.class(),
            Value::Int(_) => TYPE_CLASSES.with(|classes| classes.borrow().get(&"Int".into()).cloned()),
            _ => todo!("The others D:"),
        }
    }

    pub fn type_id(&self) -> TypeId {
        match self {
            Value::Null => NULL_TYPE_ID.with(Clone::clone),
            Value::Unit => UNIT_TYPE_ID.with(Clone::clone),
            Value::Bool(_) => BOOL_TYPE_ID.with(Clone::clone),
            Value::Float(_) => FLOAT_TYPE_ID.with(Clone::clone),
            Value::FnClosure(_) => FN_CLOSURE_TYPE_ID.with(Clone::clone),
            Value::FnScript(_) => FN_SCRIPT_TYPE_ID.with(Clone::clone),
            Value::FnNative(_) => FN_NATIVE_TYPE_ID.with(Clone::clone),
            Value::FnBound(_) => FN_BOUND_TYPE_ID.with(Clone::clone),
            Value::Array(_) => ARRAY_TYPE_ID.with(Clone::clone),
            Value::String(_) => STRING_TYPE_ID.with(Clone::clone),
            Value::Symbol(_) => SYMBOL_TYPE_ID.with(Clone::clone),
            Value::Object(object) => match object.class() {
                None => OBJECT_TYPE_ID.with(Clone::clone),
                Some(class) => class.instance_type_id(),
            },
            Value::Class(class) => class.type_id(),
            value => match value.class() {
                Some(class) => class.instance_type_id(),
                _ => unreachable!("type_id should always resolve."),
            },
        }
    }

    // TODO: Register known class types with the runtime.
    pub fn register_classes(_runtime: &mut dyn Runtime) -> Result<(), RuntimeError> {
        let mut int_class = _runtime.new_class("Int")?;

        int_class.register_native_method(
            NEW,
            Rc::new(|_, args| {
                if let [_, value, ..] = args {
                    return match value {
                        Value::Int(int) => Ok(Value::Int(*int)),
                        Value::Float(float) => Ok(Value::Int(*float as i64)),
                        _ => Ok(Value::Null),
                    };
                }

                Ok(Value::Null)
            }),
        );

        int_class.register_native_method(
            "sqrt",
            Rc::new(|_, values| {
                if let [Value::Int(value), ..] = values {
                    return Ok(Value::Int((*value as f64).sqrt() as i64));
                }

                Ok(Value::Null)
            }),
        );

        TYPE_CLASSES.with(|classes| classes.borrow_mut().insert("Int".into(), int_class.class()));

        Ok(())
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
