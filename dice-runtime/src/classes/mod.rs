use crate::{module::ModuleLoader, Runtime};

pub mod object;

mod array;
mod bool;
mod class;
mod float;
mod function;
mod int;
mod string;
mod unit;

impl<L> Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_known_types(&mut self) {
        self.register_array();
        self.register_class();
        self.register_float();
        self.register_int();
    }
}
