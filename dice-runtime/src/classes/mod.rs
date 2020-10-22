mod array;
mod class;
mod float;
mod int;
mod object;

use crate::{module::ModuleLoader, runtime::Runtime};

pub fn register(runtime: &mut Runtime<impl ModuleLoader>) {
    let object_base = object::register(runtime);
    array::register(runtime, object_base.clone());
    class::register(runtime, object_base.clone());
    float::register(runtime, object_base.clone());
    int::register(runtime, object_base.clone());
}
