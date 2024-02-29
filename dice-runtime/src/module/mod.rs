use gc_arena::Collect;

use dice_bytecode::Bytecode;
use dice_core::error::Error;

use crate::runtime::RuntimeContext;
use crate::value::Symbol;

pub mod file_loader;

#[derive(Clone, Collect)]
#[collect(require_static)]
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
    fn load_module(&mut self, ctx: &RuntimeContext<'_>, name: Symbol) -> Result<Module, Error>;
}
