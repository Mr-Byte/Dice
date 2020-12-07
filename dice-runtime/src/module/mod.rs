pub mod file_loader;

use dice_core::{bytecode::Bytecode, value::Symbol};

#[derive(Clone)]
pub struct Module {
    pub id: Symbol,
    pub bytecode: Bytecode,
}

impl Module {
    pub fn new(id: Symbol, bytecode: Bytecode) -> Self {
        Module { id, bytecode }
    }
}

pub trait ModuleLoader: Default {
    fn load_module(&mut self, name: Symbol) -> Result<Module, ()>;
}
