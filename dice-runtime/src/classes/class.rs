use crate::module::ModuleLoader;
use dice_core::{
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub fn register_class(&mut self) {
        let class = self.object_class.derive("Class");
        class.set_method("name", Rc::new(name) as NativeFn);
        class.set_method("base", Rc::new(base_class) as NativeFn);

        self.known_types.insert(ValueKind::Class, class.clone());
        self.globals.insert(class.name(), Value::Class(class.clone()));
    }
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
