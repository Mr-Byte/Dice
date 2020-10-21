use crate::module::ModuleLoader;
use dice_core::{runtime::Runtime, value::ValueKind};

pub fn register(runtime: &mut crate::Runtime<impl ModuleLoader>) {
    let class = runtime.new_class("Class").unwrap();
    runtime
        .known_type_ids
        .insert(ValueKind::Class, class.class().instance_type_id());
}
