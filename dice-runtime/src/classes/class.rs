use crate::module::ModuleLoader;
use dice_core::{
    runtime::Runtime,
    value::{Class, NativeFn, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>, base: &Class) {
    let class = base.derive("Class");
    runtime.known_types.insert(ValueKind::Class, class.clone());
    runtime.globals.insert(class.name(), Value::Class(class.clone()));

    class.set_method("name", Rc::new(name) as NativeFn);
    class.set_method("base", Rc::new(base_class) as NativeFn);
}

fn name(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Class(class), ..] => Ok(Value::with_string(class.name())),
        _ => Ok(Value::Null),
    }
}

fn base_class(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Class(class), ..] => {
            let result = class.base().map_or_else(|| Value::Null, Value::Class);
            Ok(result)
        }
        _ => Ok(Value::Null),
    }
}
