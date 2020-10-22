use crate::module::ModuleLoader;
use dice_core::runtime::ClassBuilder;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>, base: &ClassBuilder) {
    let mut float = base.derive("Float");
    runtime.known_types.insert(ValueKind::Float, float.class());

    // NOTE: This does not currently expose all possible functions rust has, just a subset.
    // If the need arises, this list can be further expanded.
    float.register_native_method(NEW, Rc::new(construct_float));
    float.register_native_method("abs", bind_f64_ret_f64(f64::abs));
    float.register_native_method("sqrt", bind_f64_ret_f64(f64::sqrt));
    float.register_native_method("cbrt", bind_f64_ret_f64(f64::cbrt));
    float.register_native_method("floor", bind_f64_ret_f64(f64::floor));
    float.register_native_method("ceil", bind_f64_ret_f64(f64::ceil));
    float.register_native_method("round", bind_f64_ret_f64(f64::round));
    float.register_native_method("cos", bind_f64_ret_f64(f64::cos));
    float.register_native_method("sin", bind_f64_ret_f64(f64::sin));
    float.register_native_method("tan", bind_f64_ret_f64(f64::tan));
    float.register_native_method("acos", bind_f64_ret_f64(f64::acos));
    float.register_native_method("asin", bind_f64_ret_f64(f64::asin));
    float.register_native_method("atan", bind_f64_ret_f64(f64::atan));
    float.register_native_method("atan2", bind_f64_f64_ret_f64(f64::atan2));
    float.register_native_method("cosh", bind_f64_ret_f64(f64::cosh));
    float.register_native_method("sinh", bind_f64_ret_f64(f64::sinh));
    float.register_native_method("tanh", bind_f64_ret_f64(f64::tanh));
    float.register_native_method("acosh", bind_f64_ret_f64(f64::acosh));
    float.register_native_method("asinh", bind_f64_ret_f64(f64::asinh));
    float.register_native_method("atanh", bind_f64_ret_f64(f64::atanh));
    float.register_native_method("log", bind_f64_f64_ret_f64(f64::log));
    float.register_native_method("log2", bind_f64_ret_f64(f64::log2));
    float.register_native_method("ln", bind_f64_ret_f64(f64::ln));
    float.register_native_method("log10", bind_f64_ret_f64(f64::log10));
    float.register_native_method("pow", bind_f64_f64_ret_f64(f64::powf));
    float.register_native_method("to_degrees", bind_f64_ret_f64(f64::to_degrees));
    float.register_native_method("to_radians", bind_f64_ret_f64(f64::to_radians));
    float.register_native_method("min", bind_f64_f64_ret_f64(f64::min));
    float.register_native_method("max", bind_f64_f64_ret_f64(f64::max));

    float.register_native_method("is_finite", bind_f64_ret_bool(f64::is_finite));
    float.register_native_method("is_infinite", bind_f64_ret_bool(f64::is_infinite));
    float.register_native_method("is_nan", bind_f64_ret_bool(f64::is_nan));
    float.register_native_method("is_sign_negative", bind_f64_ret_bool(f64::is_sign_negative));
    float.register_native_method("is_sign_positive", bind_f64_ret_bool(f64::is_sign_positive));
    float.register_native_method("is_normal", bind_f64_ret_bool(f64::is_normal));

    float.register_native_static_property("MAX", Value::Float(f64::MAX));
    float.register_native_static_property("MIN", Value::Float(f64::MIN));
    float.register_native_static_property("MIN_POSITIVE", Value::Float(f64::MIN_POSITIVE));
    float.register_native_static_property("EPSILON", Value::Float(f64::EPSILON));
    float.register_native_static_property("PI", Value::Float(std::f64::consts::PI));
    float.register_native_static_property("E", Value::Float(std::f64::consts::E));
    float.register_native_static_property("NAN", Value::Float(std::f64::NAN));
    float.register_native_static_property("INFINITY", Value::Float(std::f64::INFINITY));
    float.register_native_static_property("NEG_INFINITY", Value::Float(std::f64::NEG_INFINITY));
}

fn construct_float(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, param, ..] = args {
        match param {
            value @ Value::Float(_) => Ok(value.clone()),
            Value::Int(value) => Ok(Value::Float(*value as f64)),
            Value::String(string) => Ok(string.parse::<f64>().map_or_else(|_| Value::Null, Value::Float)),
            _ => Ok(Value::Null),
        }
    } else {
        Ok(Value::Null)
    }
}

fn bind_f64_ret_f64(
    function: impl Fn(f64) -> f64 + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(this), ..] => Ok(Value::Float(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_f64_ret_bool(
    function: impl Fn(f64) -> bool + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(this), ..] => Ok(Value::Bool(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_f64_f64_ret_f64(
    function: impl Fn(f64, f64) -> f64 + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(first), Value::Float(second), ..] => Ok(Value::Float(function(*first, *second))),
        _ => Ok(Value::Null),
    })
}
