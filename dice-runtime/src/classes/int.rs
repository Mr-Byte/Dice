use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::{ClassBuilder, Runtime},
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>, base: &ClassBuilder) {
    let mut class = base.derive("Int");
    runtime.known_types.insert(ValueKind::Int, class.class());
    runtime
        .globals
        .insert(class.class().name(), Value::Class(class.class()));

    class.register_native_method(NEW, Rc::new(construct_int));
    class.register_native_method("abs", bind_i64_ret_i64(i64::abs));
    class.register_native_method("pow", Rc::new(pow));
    class.register_native_method("is_positive", bind_i64_ret_bool(i64::is_positive));
    class.register_native_method("is_negative", bind_i64_ret_bool(i64::is_negative));
    class.register_native_method("min", bind_i64_i64_ret_i64(i64::min));
    class.register_native_method("max", bind_i64_i64_ret_i64(i64::max));

    // TODO: Decide if the wrapping/overflowing/saturating operators need included.

    class.register_native_static_property("MAX", Value::Int(i64::MAX));
    class.register_native_static_property("MIN", Value::Int(i64::MIN));
    class.register_native_static_property("I32_MAX", Value::Int(i32::MAX as i64));
    class.register_native_static_property("I32_MIN", Value::Int(i32::MIN as i64));
    class.register_native_static_property("U32_MAX", Value::Int(u32::MAX as i64));
    class.register_native_static_property("U32_MIN", Value::Int(u32::MIN as i64));
    class.register_native_static_property("I16_MAX", Value::Int(i16::MAX as i64));
    class.register_native_static_property("I16_MIN", Value::Int(i16::MIN as i64));
    class.register_native_static_property("U16_MAX", Value::Int(u16::MAX as i64));
    class.register_native_static_property("U16_MIN", Value::Int(u16::MIN as i64));
    class.register_native_static_property("I8_MAX", Value::Int(i8::MAX as i64));
    class.register_native_static_property("I8_MIN", Value::Int(i8::MIN as i64));
    class.register_native_static_property("U8_MAX", Value::Int(u8::MAX as i64));
    class.register_native_static_property("U8_MIN", Value::Int(u8::MIN as i64));
}

fn construct_int(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, param, ..] = args {
        match param {
            value @ Value::Int(_) => Ok(value.clone()),
            Value::Float(value) => Ok(Value::Int(*value as i64)),
            Value::String(string) => Ok(string.parse::<i64>().map_or_else(|_| Value::Null, Value::Int)),
            _ => Ok(Value::Null),
        }
    } else {
        Ok(Value::Null)
    }
}

fn bind_i64_ret_i64(
    function: impl Fn(i64) -> i64 + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(this), ..] => Ok(Value::Int(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_i64_ret_bool(
    function: impl Fn(i64) -> bool + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(this), ..] => Ok(Value::Bool(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_i64_i64_ret_i64(
    function: impl Fn(i64, i64) -> i64 + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(first), Value::Int(second), ..] => Ok(Value::Int(function(*first, *second))),
        _ => Ok(Value::Null),
    })
}

fn pow(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), Value::Int(exp), ..] = args {
        Ok(Value::Int(this.pow(*exp as u32)))
    } else {
        Ok(Value::Null)
    }
}
