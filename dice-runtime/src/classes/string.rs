use crate::module::ModuleLoader;
use dice_core::error::Error;
use dice_core::{
    protocol::{class::NEW, operator::ADD},
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_string(&mut self) {
        let class = self.any_class.derive("String");

        class.set_method(&NEW, Rc::new(construct_string) as NativeFn);
        class.set_method(&ADD, Rc::new(concat) as NativeFn);

        // TODO: Figure out what methods to expose for strings.

        self.set_value_class(ValueKind::String, class);
    }
}

fn construct_string(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [_, param, ..] => Ok(Value::with_string(param.to_string())),
        _ => Ok(Value::Null),
    }
}
fn concat(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::String(this), Value::String(other), ..] => Ok(Value::with_string(format!("{}{}", &*this, &*other))),
        _ => Ok(Value::Null),
    }
}
