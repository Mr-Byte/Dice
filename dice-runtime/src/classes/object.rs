use crate::module::ModuleLoader;
use dice_core::runtime::ClassBuilder;
use dice_core::{runtime::Runtime, value::ValueKind};

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) -> ClassBuilder {
    let object = runtime.new_class("Object").unwrap();
    runtime.known_types.insert(ValueKind::Object, object.class());

    object
}
