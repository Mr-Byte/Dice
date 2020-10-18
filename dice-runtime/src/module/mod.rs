pub mod file_loader;

use dice_core::bytecode::Bytecode;
use dice_core::value::Symbol;
use dice_error::runtime_error::RuntimeError;

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
    fn load_module(&mut self, name: Symbol) -> Result<Module, RuntimeError>;
}
