use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_string(&mut self) {
        let class = self.any_class.derive("String");

        class.set_method(NEW, Rc::new(construct_string) as NativeFn);

        self.set_value_class(ValueKind::String, class);
    }
}

fn construct_string(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [_, param, ..] => Ok(Value::with_string(param.to_string())),
        _ => Ok(Value::Null),
    }
}
