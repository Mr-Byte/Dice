use crate::module::{Module, ModuleLoader};
use dice_compiler::compiler::Compiler;
use dice_core::{
    source::{Source, SourceKind},
    value::Symbol,
};
use dice_error::runtime_error::RuntimeError;

#[derive(Default)]
pub struct FileModuleLoader;

impl ModuleLoader for FileModuleLoader {
    fn load_module(&mut self, name: Symbol) -> Result<Module, RuntimeError> {
        let path = std::fs::canonicalize(&*name)?;
        let working_dir = std::fs::canonicalize(std::env::current_dir()?)?;

        // TODO: Have a way to set the modules root as a part of the runtime.
        if !path.starts_with(working_dir) {
            todo!("Error about not being able to read outside the scripts directory.")
        }

        let source = std::fs::read_to_string(&path)?;
        let source = Source::with_path(source, path.to_string_lossy().into(), SourceKind::Module);
        let module = Compiler::compile(source)?;
        let module = Module::new(name, module);

        Ok(module)
    }
}
