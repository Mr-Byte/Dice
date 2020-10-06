pub mod file_loader;

use dice_core::bytecode::Bytecode;
use dice_error::runtime_error::RuntimeError;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ModuleId(u64);

#[derive(Clone)]
pub struct Module {
    pub id: ModuleId,
    pub bytecode: Bytecode,
}

impl Module {
    pub fn new(id: ModuleId, bytecode: Bytecode) -> Self {
        Module { id, bytecode }
    }
}

impl From<u64> for ModuleId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

pub trait ModuleLoader: Default {
    fn load_module(&mut self, path: &str) -> Result<Module, RuntimeError>;
}
