use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::{ClassBuilder, Runtime},
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>, base: &ClassBuilder) {
    let mut class = base.derive("Array");
    runtime.known_types.insert(ValueKind::Array, class.class());
    runtime
        .globals
        .insert(class.class().name(), Value::Class(class.class()));

    class.register_native_method(NEW, Rc::new(construct_array));
    class.register_native_method("push", Rc::new(push));
    class.register_native_method("pop", Rc::new(pop));
    class.register_native_method("length", Rc::new(length));
}

fn construct_array(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, rest @ ..] = args {
        let arr = rest.to_vec();

        Ok(Value::Array(arr.into()))
    } else {
        Ok(Value::Null)
    }
}

fn push(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), param, ..] = args {
        arr.elements_mut().push(param.clone());

        Ok(Value::Unit)
    } else {
        Ok(Value::Null)
    }
}

fn pop(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), ..] = args {
        let result = arr.elements_mut().pop().unwrap_or_default();

        Ok(result)
    } else {
        Ok(Value::Null)
    }
}

fn length(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), ..] = args {
        Ok(Value::Int(arr.elements().len() as i64))
    } else {
        Ok(Value::Null)
    }
}
