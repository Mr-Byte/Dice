use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub fn register_float(&mut self) {
        let class = self.any_class.derive("Float");

        // NOTE: This does not currently expose all possible functions rust has, just a subset.
        // If the need arises, this list can be further expanded.
        class.set_method(NEW, Rc::new(construct_float) as NativeFn);
        class.set_method("abs", bind_f64_ret_f64(f64::abs));
        class.set_method("sqrt", bind_f64_ret_f64(f64::sqrt));
        class.set_method("cbrt", bind_f64_ret_f64(f64::cbrt));
        class.set_method("floor", bind_f64_ret_f64(f64::floor));
        class.set_method("ceil", bind_f64_ret_f64(f64::ceil));
        class.set_method("round", bind_f64_ret_f64(f64::round));
        class.set_method("cos", bind_f64_ret_f64(f64::cos));
        class.set_method("sin", bind_f64_ret_f64(f64::sin));
        class.set_method("tan", bind_f64_ret_f64(f64::tan));
        class.set_method("acos", bind_f64_ret_f64(f64::acos));
        class.set_method("asin", bind_f64_ret_f64(f64::asin));
        class.set_method("atan", bind_f64_ret_f64(f64::atan));
        class.set_method("atan2", bind_f64_f64_ret_f64(f64::atan2));
        class.set_method("cosh", bind_f64_ret_f64(f64::cosh));
        class.set_method("sinh", bind_f64_ret_f64(f64::sinh));
        class.set_method("tanh", bind_f64_ret_f64(f64::tanh));
        class.set_method("acosh", bind_f64_ret_f64(f64::acosh));
        class.set_method("asinh", bind_f64_ret_f64(f64::asinh));
        class.set_method("atanh", bind_f64_ret_f64(f64::atanh));
        class.set_method("log", bind_f64_f64_ret_f64(f64::log));
        class.set_method("log2", bind_f64_ret_f64(f64::log2));
        class.set_method("ln", bind_f64_ret_f64(f64::ln));
        class.set_method("log10", bind_f64_ret_f64(f64::log10));
        class.set_method("pow", bind_f64_f64_ret_f64(f64::powf));
        class.set_method("to_degrees", bind_f64_ret_f64(f64::to_degrees));
        class.set_method("to_radians", bind_f64_ret_f64(f64::to_radians));
        class.set_method("min", bind_f64_f64_ret_f64(f64::min));
        class.set_method("max", bind_f64_f64_ret_f64(f64::max));

        class.set_method("is_finite", bind_f64_ret_bool(f64::is_finite));
        class.set_method("is_infinite", bind_f64_ret_bool(f64::is_infinite));
        class.set_method("is_nan", bind_f64_ret_bool(f64::is_nan));
        class.set_method("is_sign_negative", bind_f64_ret_bool(f64::is_sign_negative));
        class.set_method("is_sign_positive", bind_f64_ret_bool(f64::is_sign_positive));
        class.set_method("is_normal", bind_f64_ret_bool(f64::is_normal));

        class.set_field("MAX", Value::Float(f64::MAX));
        class.set_field("MIN", Value::Float(f64::MIN));
        class.set_field("MIN_POSITIVE", Value::Float(f64::MIN_POSITIVE));
        class.set_field("EPSILON", Value::Float(f64::EPSILON));
        class.set_field("PI", Value::Float(std::f64::consts::PI));
        class.set_field("E", Value::Float(std::f64::consts::E));
        class.set_field("NAN", Value::Float(std::f64::NAN));
        class.set_field("INFINITY", Value::Float(std::f64::INFINITY));
        class.set_field("NEG_INFINITY", Value::Float(std::f64::NEG_INFINITY));

        self.set_value_class(ValueKind::Float, class);
    }
}

fn construct_float(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [_, param, ..] => match param {
            value @ Value::Float(_) => Ok(value.clone()),
            Value::Int(value) => Ok(Value::Float(*value as f64)),
            Value::String(string) => Ok(string.parse::<f64>().map_or_else(|_| Value::Null, Value::Float)),
            _ => Ok(Value::Null),
        },
        _ => Ok(Value::Null),
    }
}

fn bind_f64_ret_f64(function: impl Fn(f64) -> f64 + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(this), ..] => Ok(Value::Float(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_f64_ret_bool(function: impl Fn(f64) -> bool + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(this), ..] => Ok(Value::Bool(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_f64_f64_ret_f64(function: impl Fn(f64, f64) -> f64 + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Float(first), Value::Float(second), ..] => Ok(Value::Float(function(*first, *second))),
        _ => Ok(Value::Null),
    })
}
