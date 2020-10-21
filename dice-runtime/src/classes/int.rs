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
    int.register_native_method("pow", Rc::new(pow));
    int.register_native_method("is_positive", Rc::new(is_positive));
    int.register_native_method("is_negative", Rc::new(is_negative));
    // TODO: Decide if the wrapping/overflowing/saturating operators need included.

    int.register_native_static_property("MAX", Value::Int(i64::MAX));
    int.register_native_static_property("MIN", Value::Int(i64::MIN));
    int.register_native_static_property("I32_MAX", Value::Int(i32::MAX as i64));
    int.register_native_static_property("I32_MIN", Value::Int(i32::MIN as i64));
    int.register_native_static_property("U32_MAX", Value::Int(u32::MAX as i64));
    int.register_native_static_property("U32_MIN", Value::Int(u32::MIN as i64));
    int.register_native_static_property("I16_MAX", Value::Int(i16::MAX as i64));
    int.register_native_static_property("I16_MIN", Value::Int(i16::MIN as i64));
    int.register_native_static_property("U16_MAX", Value::Int(u16::MAX as i64));
    int.register_native_static_property("U16_MIN", Value::Int(u16::MIN as i64));
    int.register_native_static_property("I8_MAX", Value::Int(i8::MAX as i64));
    int.register_native_static_property("I8_MIN", Value::Int(i8::MIN as i64));
    int.register_native_static_property("U8_MAX", Value::Int(u8::MAX as i64));
    int.register_native_static_property("U8_MIN", Value::Int(u8::MIN as i64));
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

fn pow(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), Value::Int(exp), ..] = args {
        Ok(Value::Int(this.pow(*exp as u32)))
    } else {
        Ok(Value::Null)
    }
}

fn is_positive(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), ..] = args {
        Ok(Value::Bool(this.is_positive()))
    } else {
        Ok(Value::Null)
    }
}

fn is_negative(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), ..] = args {
        Ok(Value::Bool(this.is_negative()))
    } else {
        Ok(Value::Null)
    }
}
