use crate::module::{Module, ModuleLoader};
use dice_compiler::compiler::Compiler;
use dice_core::source::{Source, SourceKind};
use dice_core::value::Symbol;
use dice_error::runtime_error::RuntimeError;

#[derive(Default)]
pub struct FileModuleLoader;

impl ModuleLoader for FileModuleLoader {
    fn load_module(&mut self, name: Symbol) -> Result<Module, RuntimeError> {
        let path = std::fs::canonicalize(&*name)?;
        let source = std::fs::read_to_string(&path)?;
        let source = Source::with_path(source, path.to_string_lossy().into(), SourceKind::Module);
        let module = Compiler::compile(source)?;
        let module = Module::new(name, module);

        Ok(module)
    }
}
