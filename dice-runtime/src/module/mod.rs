pub mod file_loader;

use dice_core::{bytecode::Bytecode, error::Error, value::Symbol};
use gc_arena::Collect;

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct Module<'gc> {
    pub id: Symbol,
    pub bytecode: Bytecode<'gc>,
}

impl<'gc> Module<'gc> {
    pub fn new(id: Symbol, bytecode: Bytecode<'gc>) -> Self {
        Module { id, bytecode }
    }
}

pub trait ModuleLoader: Default {
    fn load_module(&mut self, name: Symbol) -> Result<Module, Error>;
}
