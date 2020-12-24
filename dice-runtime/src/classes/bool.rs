use crate::module::ModuleLoader;
use dice_core::{
    error::Error,
    protocol::class::NEW,
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_bool(&mut self) {
        let class = self.any_class.derive("Bool");

        class.set_method(&NEW, Box::new(construct_bool) as NativeFn);

        self.set_value_class(ValueKind::Bool, class);
    }
}

fn construct_bool(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [_, param, ..] => match param {
            value @ Value::Bool(_) => Ok(value.clone()),
            Value::String(string) => Ok(string.parse::<bool>().map_or_else(|_| Value::Null, Value::Bool)),
            _ => Ok(Value::Null),
        },
        _ => Ok(Value::Null),
    }
}
