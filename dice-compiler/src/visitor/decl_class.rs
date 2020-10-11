use crate::compiler::Compiler;
use crate::scope_stack::State;
use crate::visitor::NodeVisitor;
use dice_error::compiler_error::CompilerError;
use dice_syntax::ClassDecl;

impl NodeVisitor<&ClassDecl> for Compiler {
    fn visit(&mut self, node: &ClassDecl) -> Result<(), CompilerError> {
        let source = self.source.clone();
        let slot = {
            let class_name = node.name.clone();
            let local = self.context()?.scope_stack().local(&class_name).ok_or_else(|| {
                CompilerError::InternalCompilerError(String::from("Class not already declared in scope."))
            })?;

            // NOTE: Check if a class of the given name has already been initialized.
            if let State::Class { ref mut is_initialized } = &mut local.state {
                if *is_initialized {
                    return Err(CompilerError::ItemAlreadyDeclared(class_name));
                }

                *is_initialized = true;
            }

            local.slot as u8
        };

        emit_bytecode! {
            self.assembler()?, node.span => [
                CREATE_CLASS &node.name, source.path();
                STORE_LOCAL slot;
            ]
        }

        Ok(())
    }
}
