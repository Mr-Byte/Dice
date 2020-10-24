mod array;
mod bool;
mod class;
mod float;
mod function;
mod int;
mod object;
mod string;
mod unit;

use crate::{module::ModuleLoader, runtime::Runtime};

pub fn register(runtime: &mut Runtime<impl ModuleLoader>) {
    let base = object::register(runtime);
    array::register(runtime, &base);
    class::register(runtime, &base);
    float::register(runtime, &base);
    int::register(runtime, &base);
}
