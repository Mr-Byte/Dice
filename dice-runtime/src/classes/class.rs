use crate::module::ModuleLoader;
use dice_core::{
    error::Error,
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub fn register_class(&mut self) {
        let class = self.any_class.derive("Class");
        class.set_method("name", Box::new(name) as NativeFn);
        class.set_method("base", Box::new(base_class) as NativeFn);

        self.set_value_class(ValueKind::Class, class);
    }
}

fn name(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Class(class), ..] => Ok(Value::with_string(class.name())),
        _ => Ok(Value::Null),
    }
}

fn base_class(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Class(class), ..] => {
            let result = class.base().map_or_else(|| Value::Null, Value::Class);
            Ok(result)
        }
        _ => Ok(Value::Null),
    }
}
