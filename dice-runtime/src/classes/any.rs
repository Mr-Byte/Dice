use crate::module::ModuleLoader;
use dice_core::{
    protocol::object::{ANY_CLASS, TO_STRING},
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
        let class = Class::new(ANY_CLASS.into());

        class.set_method(TO_STRING, Rc::new(to_string) as NativeFn);

        class
    }
}

fn to_string(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [value, ..] => Ok(Value::with_string(format!("{}", value))),
        _ => Ok(Value::Null),
    }
}
