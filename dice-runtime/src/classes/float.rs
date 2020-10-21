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
    float.register_native_method("abs", Rc::new(abs));
    float.register_native_method("sqrt", Rc::new(sqrt));
    float.register_native_method("cos", Rc::new(cos));
    float.register_native_method("sin", Rc::new(sin));

    float.register_native_static_property("MAX", Value::Float(f64::MAX));
    float.register_native_static_property("MIN", Value::Float(f64::MIN));
    float.register_native_static_property("EPSILON", Value::Float(f64::EPSILON));
    float.register_native_static_property("PI", Value::Float(std::f64::consts::PI));
    float.register_native_static_property("E", Value::Float(std::f64::consts::E));

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

fn abs(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Float(this), ..] = args {
        Ok(Value::Float(this.abs()))
    } else {
        Ok(Value::Null)
    }
}

fn sqrt(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Float(this), ..] = args {
        Ok(Value::Float(this.sqrt()))
    } else {
        Ok(Value::Null)
    }
}

fn cos(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Float(this), ..] = args {
        Ok(Value::Float(this.cos()))
    } else {
        Ok(Value::Null)
    }
}

fn sin(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Float(this), ..] = args {
        Ok(Value::Float(this.sin()))
    } else {
        Ok(Value::Null)
    }
}
