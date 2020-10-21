use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) {
    let mut int = runtime.new_class("Int").unwrap();
    runtime
        .known_type_ids
        .insert(ValueKind::Int, int.class().instance_type_id());
    runtime.known_types.insert(ValueKind::Int, int.class());

    int.register_native_method(NEW, Rc::new(construct_int));
    int.register_native_method("abs", Rc::new(abs));

    int.register_native_static_property("MAX", Value::Int(i64::MAX));
    int.register_native_static_property("MIN", Value::Int(i64::MIN));
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

fn abs(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), ..] = args {
        Ok(Value::Int(this.abs()))
    } else {
        Ok(Value::Null)
    }
}
