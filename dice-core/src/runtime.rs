use gc_arena::Mutation;

use crate::{
    error::Error,
    value::{Class, Object, SymbolInterner, Value},
};

pub trait Runtime<'gc> {
    fn new_module(&mut self, name: &str) -> Result<Object<'gc>, Error>;
    fn new_class(&mut self, name: &str) -> Result<Class<'gc>, Error>;
    fn new_object(&mut self) -> Result<Object<'gc>, Error>;

    fn load_prelude(&mut self, path: &str) -> Result<(), Error>;
    fn add_global(&mut self, name: &str, value: Value) -> Result<(), Error>;

    fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value<'gc>, Error>;

    fn any_class(&self) -> Result<Class<'gc>, Error>;
    fn class_of(&self, value: &Value) -> Result<Class<'gc>, Error>;
    fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, Error>;
}

pub struct RuntimeContext<'gc, State> {
    pub mutation: &'gc Mutation<'gc>,
    pub interner: &'gc SymbolInterner,
    pub state: &'gc State,
}
