use crate::module::ModuleLoader;
use dice_core::protocol::class::NEW;
use dice_core::runtime::Runtime;
use dice_core::value::{Class, Object, Value, ValueKind};
use dice_error::runtime_error::RuntimeError;

use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) -> Class {
    let mut object = runtime.new_class("Object", None).unwrap();
    runtime.known_types.insert(ValueKind::Object, object.class());

    let class = object.class();
    object.register_native_method(
        NEW,
        Rc::new(move |_, _| Ok(Value::Object(Object::new(class.instance_type_id(), class.clone())))),
    );
    object.register_native_method("to_string", Rc::new(to_string));
    object.register_native_method("equals", Rc::new(equals));

    // TODO: Figure out how to implement get_hashcode.

    return object.class();
}

fn to_string(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [this, ..] => Ok(Value::with_string(format!("{}", this))),
        _ => Ok(Value::Null),
    }
}

fn equals(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [this, other, ..] => Ok(Value::Bool(this == other)),
        _ => Ok(Value::Null),
    }
}
