use crate::module::ModuleLoader;
use dice_core::protocol::object::TO_STRING;
use dice_core::value::{Class, NativeFn};
use dice_core::{
    runtime::Runtime,
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) -> Class {
    let class = runtime.new_class("Object", None).unwrap();
    runtime.known_types.insert(ValueKind::Object, class.clone());
    runtime.globals.insert(class.name(), Value::Class(class.clone()));

    class.set_method(TO_STRING, Rc::new(to_string) as NativeFn);

    class
}

fn to_string(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [value, ..] => Ok(Value::with_string(format!("{}", value))),
        _ => Ok(Value::Null),
    }
}
