mod array;
mod class;
mod float;
mod int;
mod object;

use crate::{module::ModuleLoader, runtime::Runtime};

pub fn register(runtime: &mut Runtime<impl ModuleLoader>) {
    let base = object::register(runtime);
    array::register(runtime, &base);
    class::register(runtime, &base);
    float::register(runtime, &base);
    int::register(runtime, &base);
}
