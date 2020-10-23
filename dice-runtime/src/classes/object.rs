use crate::module::ModuleLoader;
use dice_core::runtime::ClassBuilder;
use dice_core::value::Value;
use dice_core::{runtime::Runtime, value::ValueKind};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) -> ClassBuilder {
    let mut class = runtime.new_class("Object").unwrap();
    runtime.known_types.insert(ValueKind::Object, class.class());
    runtime
        .globals
        .insert(class.class().name(), Value::Class(class.class()));

    class.register_native_method("to_string", Rc::new(to_string));

    class
}

fn to_string(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [value, ..] => Ok(Value::with_string(format!("{}", value))),
        _ => Ok(Value::Null),
    }
}
