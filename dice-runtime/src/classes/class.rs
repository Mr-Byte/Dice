use crate::module::ModuleLoader;
use dice_core::{
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub fn register_class(&mut self) {
        let class = self.any_class.derive("Class");
        class.set_method("name", Rc::new(name) as NativeFn);
        class.set_method("base", Rc::new(base_class) as NativeFn);

        self.set_value_class(ValueKind::Class, class);
    }
}

fn name(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, ()> {
    match args {
        [Value::Class(class), ..] => Ok(Value::with_string(class.name())),
        _ => Ok(Value::Null),
    }
}

fn base_class(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, ()> {
    match args {
        [Value::Class(class), ..] => {
            let result = class.base().map_or_else(|| Value::Null, Value::Class);
            Ok(result)
        }
        _ => Ok(Value::Null),
    }
}
