use crate::module::ModuleLoader;
use dice_core::runtime::Runtime;
use dice_core::value::ValueKind;

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) {
    let object = runtime.new_class("Object").unwrap();
    runtime.known_types.insert(ValueKind::Object, object.class());
}
