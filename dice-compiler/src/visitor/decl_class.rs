use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
    visitor::{FnKind, NodeVisitor},
};
use dice_core::protocol::class::{NEW, SELF};
use dice_core::protocol::ProtocolSymbol;
use dice_core::value::Symbol;
use dice_error::compiler_error::CompilerError;
use dice_syntax::{ClassDecl, SyntaxNode};

impl NodeVisitor<&ClassDecl> for Compiler {
    fn visit(&mut self, node: &ClassDecl) -> Result<(), CompilerError> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);

        let slot = {
            let class_name: Symbol = (&*node.name).into();
            let local = self.context()?.scope_stack().local(class_name.clone()).ok_or_else(|| {
                CompilerError::InternalCompilerError(String::from("Class not already declared in scope."))
            })?;

            // NOTE: Check if a class of the given name has already been initialized.
            if let State::Class { ref mut is_initialized } = &mut local.state {
                if *is_initialized {
                    return Err(CompilerError::ItemAlreadyDeclared(node.name.to_owned()));
                }

                *is_initialized = true;
            }

            local.slot as u8
        };

        emit_bytecode! {
            self.assembler()?, node.span => [
                CREATE_CLASS &node.name;
                STORE_LOCAL slot;
            ]
        }

        for associated_item in node.associated_items.iter().copied() {
            let node = self.syntax_tree.get(associated_item);

            match node {
                SyntaxNode::FnDecl(fn_decl) => {
                    let fn_decl = fn_decl.clone();
                    let is_method = fn_decl.args.first().map(|arg| arg == &*SELF.get()).unwrap_or(false);
                    let kind = if is_method {
                        if fn_decl.name == &*NEW.get() {
                            FnKind::Constructor
                        } else {
                            FnKind::Method
                        }
                    } else {
                        if fn_decl.name == &*NEW.get() {
                            return Err(CompilerError::NewMustHaveSelfReceiver(fn_decl.span));
                        }

                        FnKind::StaticMethod
                    };

                    self.visit((&fn_decl, kind))?;

                    emit_bytecode! {
                        self.assembler()?, fn_decl.span => [
                            if is_method => [
                                STORE_METHOD &*fn_decl.name;
                                LOAD_LOCAL slot;
                            ] else [
                                STORE_FIELD &*fn_decl.name;
                                POP;
                                LOAD_LOCAL slot;
                            ]
                        ]
                    }
                }
                _ => unreachable!("Unexpected node kind encountered."),
            }
        }

        self.context()?.scope_stack().pop_scope()?;

        Ok(())
    }
}
