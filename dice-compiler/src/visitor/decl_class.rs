use crate::{
    compiler::Compiler,
    scope_stack::State,
    visitor::{FnKind, NodeVisitor},
};
use dice_core::constants::SELF;
use dice_error::compiler_error::CompilerError;
use dice_syntax::{ClassDecl, SyntaxNode};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ClassKind {
    TopLevel,
    Nested,
}

impl NodeVisitor<(&ClassDecl, ClassKind)> for Compiler {
    fn visit(&mut self, (node, kind): (&ClassDecl, ClassKind)) -> Result<(), CompilerError> {
        let source = self.source.clone();

        let slot = {
            let class_name = node.name.clone();

            match kind {
                ClassKind::TopLevel => {
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
                }
                ClassKind::Nested => self
                    .context()?
                    .scope_stack()
                    .add_local(&class_name, State::initialized(false))? as u8,
            }
        };

        emit_bytecode! {
            self.assembler()?, node.span => [
                CREATE_CLASS &node.name, source.path();
                STORE_LOCAL slot;
            ]
        }

        for associated_item in node.associated_items.iter().copied() {
            let node = self.syntax_tree.get(associated_item);

            match node {
                SyntaxNode::FnDecl(fn_decl) => {
                    let fn_decl = fn_decl.clone();
                    let is_method = fn_decl.args.first().map(|arg| arg == SELF).unwrap_or(false);

                    self.visit((&fn_decl, FnKind::Method))?;

                    emit_bytecode! {
                        self.assembler()?, fn_decl.span => [
                            if is_method => [
                                STORE_METHOD &fn_decl.name;
                                LOAD_LOCAL slot;
                            ] else [
                                STORE_FIELD &fn_decl.name;
                                POP;
                                LOAD_LOCAL slot;
                            ]
                        ]
                    }
                }
                SyntaxNode::ClassDecl(class_decl) => {
                    let class_decl = class_decl.clone();
                    self.visit((&class_decl, ClassKind::Nested))?;

                    emit_bytecode! {
                        self.assembler()?, class_decl.span => [
                            STORE_FIELD &class_decl.name;
                            POP;
                            LOAD_LOCAL slot;
                        ]
                    }
                }
                _ => unreachable!("Unexpected node kind encountered."),
            }
        }

        Ok(())
    }
}
