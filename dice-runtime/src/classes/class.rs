use crate::module::ModuleLoader;
use dice_core::runtime::ClassBuilder;
use dice_core::{
    runtime::Runtime,
    value::{Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>, base: &ClassBuilder) {
    let mut class = base.derive("Class");
    runtime.known_types.insert(ValueKind::Class, class.class());

    class.register_native_method("name", Rc::new(name));
}

fn name(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Class(class), ..] => Ok(Value::with_string(class.name())),
        _ => Ok(Value::Null),
    }
}
