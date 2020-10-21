use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) {
    let mut float = runtime.new_class("Float").unwrap();
    runtime
        .known_type_ids
        .insert(ValueKind::Float, float.class().instance_type_id());
    runtime.known_types.insert(ValueKind::Float, float.class());

    float.register_native_method(NEW, Rc::new(construct_float));
    float.register_native_method("abs", bind_f64(f64::abs));
    float.register_native_method("sqrt", bind_f64(f64::sqrt));
    float.register_native_method("floor", bind_f64(f64::floor));
    float.register_native_method("ceil", bind_f64(f64::ceil));
    float.register_native_method("round", bind_f64(f64::round));
    float.register_native_method("cos", bind_f64(f64::cos));
    float.register_native_method("sin", bind_f64(f64::sin));

    float.register_native_static_property("MAX", Value::Float(f64::MAX));
    float.register_native_static_property("MIN", Value::Float(f64::MIN));
    float.register_native_static_property("EPSILON", Value::Float(f64::EPSILON));
    float.register_native_static_property("PI", Value::Float(std::f64::consts::PI));
    float.register_native_static_property("E", Value::Float(std::f64::consts::E));
    float.register_native_static_property("NAN", Value::Float(std::f64::NAN));
    float.register_native_static_property("INFINITY", Value::Float(std::f64::INFINITY));
    float.register_native_static_property("NEG_INFINITY", Value::Float(std::f64::NEG_INFINITY));

    // TODO: Finish out the entire suite of mathematical functions for f64.
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

fn bind_f64(
    function: impl Fn(f64) -> f64 + 'static,
) -> Rc<dyn Fn(&mut dyn Runtime, &[Value]) -> Result<Value, RuntimeError>> {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| {
        if let [Value::Float(this), ..] = args {
            Ok(Value::Float(function(*this)))
        } else {
            Ok(Value::Null)
        }
    })
}
