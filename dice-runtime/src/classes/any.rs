use crate::module::ModuleLoader;
use dice_core::{
    protocol::{
        object::{ANY_CLASS, TO_STRING},
        ProtocolSymbol,
    },
    runtime::Runtime,
    value::{Class, NativeFn, Value},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub fn new_any_class() -> Class {
        let class = Class::new(ANY_CLASS.get());

        class.set_method(TO_STRING.get(), Rc::new(to_string) as NativeFn);
        class.set_method("fields", Rc::new(fields) as NativeFn);
        class.set_method("methods", Rc::new(methods) as NativeFn);
        class.set_method("class_of", Rc::new(class_of) as NativeFn);
        class
    }
}

fn to_string(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [value, ..] => Ok(Value::with_string(format!("{}", value))),
        _ => Ok(Value::Null),
    }
}

fn fields(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    let result = args
        .first()
        .and_then(|value| value.as_object().ok())
        .map_or(Value::Null, |object| {
            let fields = object.fields().keys().map(Value::with_string).collect::<Vec<_>>();

            Value::with_vec(fields)
        });

    Ok(result)
}

fn methods(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [this, ..] => {
            let class = runtime.class_of(this)?;
            let result = class
                .methods()
                .iter()
                .map(|(key, _)| Value::with_string(key))
                .collect::<Vec<_>>();

            Ok(Value::with_vec(result))
        }
        _ => Ok(Value::Null),
    }
}

fn class_of(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [this, ..] => Ok(Value::Class(runtime.class_of(this)?)),
        _ => Ok(Value::Null),
    }
}
