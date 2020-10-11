use crate::module::{Module, ModuleLoader};
use dice_compiler::compiler::Compiler;
use dice_core::source::{Source, SourceKind};
use dice_error::runtime_error::RuntimeError;
use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

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
                let source = std::fs::read_to_string(&path)?;
                let source = Source::with_path(source, path.to_string_lossy().into(), SourceKind::Module);
                let module = Compiler::compile(source)?;
                let module_id = self.module_counter.into();
                self.module_counter += 1;

                let module = Module::new(module_id, module);
                entry.insert(module.clone());
                Ok(module)
            }
        }
    }
}
