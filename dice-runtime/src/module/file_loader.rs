use crate::module::{Module, ModuleLoader};
use dice_compiler::compiler::Compiler;
use dice_core::{
    source::{Source, SourceKind},
    value::Symbol,
};

#[derive(Default)]
pub struct FileModuleLoader;

impl ModuleLoader for FileModuleLoader {
    fn load_module(&mut self, name: Symbol) -> Result<Module, ()> {
        let path = std::fs::canonicalize(&*name).expect("Error conversion");
        let working_dir =
            std::fs::canonicalize(std::env::current_dir().expect("Error conversion")).expect("Error conversion");

        // TODO: Have a way to set the modules root as a part of the runtime.
        if !path.starts_with(working_dir) {
            todo!("Error about not being able to read outside the scripts directory.")
        }

        let source = std::fs::read_to_string(&path).expect("Error conversion");
        let source = Source::with_path(source, path.to_string_lossy(), SourceKind::Module);
        let module = Compiler::compile(&source).expect("Error conversion");
        let module = Module::new(name, module);

        Ok(module)
    }
}
