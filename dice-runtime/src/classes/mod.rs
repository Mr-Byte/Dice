mod array;
mod class;
mod float;
mod int;
mod object;

use crate::{module::ModuleLoader, runtime::Runtime};

pub fn register(runtime: &mut Runtime<impl ModuleLoader>) {
    object::register(runtime);
    array::register(runtime);
    class::register(runtime);
    float::register(runtime);
    int::register(runtime);
}
