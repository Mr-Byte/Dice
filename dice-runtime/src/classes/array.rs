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
    class.register_native_method("first", Rc::new(first));
    class.register_native_method("filter", Rc::new(filter));
    class.register_native_method("map", Rc::new(map));
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

fn first(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), predicate, ..] => Ok(arr
            .elements()
            .iter()
            .find(|value| {
                runtime
                    .call_function(predicate.clone(), &[(*value).clone()])
                    .ok()
                    .and_then(|value| value.as_bool().ok())
                    .unwrap_or(false)
            })
            .cloned()
            .unwrap_or(Value::Null)),
        [Value::Array(arr), ..] => Ok(arr.elements().first().cloned().unwrap_or(Value::Null)),
        _ => Ok(Value::Null),
    }
}

fn filter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), predicate, ..] => Ok(Value::Array(
            arr.elements()
                .iter()
                .filter(|value| {
                    runtime
                        .call_function(predicate.clone(), &[(*value).clone()])
                        .ok()
                        .and_then(|value| value.as_bool().ok())
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
                .into(),
        )),
        _ => Ok(Value::Null),
    }
}

fn map(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), selector, ..] => Ok(Value::Array(
            arr.elements()
                .iter()
                .map(|value| {
                    runtime
                        .call_function(selector.clone(), &[(*value).clone()])
                        .ok()
                        .unwrap_or(Value::Null)
                })
                .collect::<Vec<_>>()
                .into(),
        )),
        _ => Ok(Value::Null),
    }
}
