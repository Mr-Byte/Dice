mod class;
mod float;
mod int;

use crate::{module::ModuleLoader, runtime::Runtime};

pub fn register(runtime: &mut Runtime<impl ModuleLoader>) {
    int::register(runtime);
    float::register(runtime);
    class::register(runtime);
}
