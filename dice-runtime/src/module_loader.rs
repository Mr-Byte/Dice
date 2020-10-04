use crate::error::RuntimeError;
use dice_compiler::compiler::Compiler;
use dice_core::bytecode::Bytecode;
use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

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

pub trait ModuleLoader {
    fn load_module(&mut self, path: &str) -> Result<Module, RuntimeError>;
}

#[derive(Default)]
pub struct FileModuleLoader {
    cached_modules: HashMap<PathBuf, Module>,
    module_counter: u64,
}

impl ModuleLoader for FileModuleLoader {
    fn load_module(&mut self, path: &str) -> Result<Module, RuntimeError> {
        let path = std::fs::canonicalize(path)?;

        match self.cached_modules.entry(path.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let module = Compiler::compile_module(&path)?;
                let module_id = self.module_counter.into();
                self.module_counter += 1;

                let module = Module::new(module_id, module);
                entry.insert(module.clone());
                Ok(module)
            }
        }
    }
}
