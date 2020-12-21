use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
    visitor::{decl_op::OpKind, ClassKind, FnKind, NodeVisitor},
};
use dice_core::{
    error::{
        codes::{CLASS_ALREADY_DECLARED, INTERNAL_COMPILER_ERROR, METHOD_RECEIVER_CANNOT_HAVE_TYPE, NEW_METHOD_MUST_HAVE_RECEIVER},
        Error,
    },
    protocol::{
        class::{NEW, SELF, SUPER},
        object::ANY_CLASS,
        ProtocolSymbol,
    },
    tags,
    value::Symbol,
};
use dice_syntax::{ClassDecl, FnDecl, LitIdent, OpDecl, SyntaxNode};

impl NodeVisitor<&ClassDecl> for Compiler {
    fn visit(&mut self, node: &ClassDecl) -> Result<(), Error> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);

        let class_kind = if let Some(base) = node.base {
            self.visit(base)?;
            ClassKind::Derived
        } else {
            self.visit(&LitIdent {
                identifier: ANY_CLASS.get().to_string(),
                span: node.span,
            })?;
            ClassKind::Base
        };

        let super_slot = self.context()?.scope_stack().add_local(SUPER.get(), State::initialized(true))? as u8;

        emit_bytecode! {
            self.assembler()?, node.span => [
                 STORE_LOCAL super_slot;
            ]
        }

        let slot = {
            let class_name: Symbol = (&*node.name.identifier).into();
            let local = self
                .context()?
                .scope_stack()
                .local(class_name)
                .ok_or_else(|| Error::new(INTERNAL_COMPILER_ERROR))?;

            // NOTE: Check if a class of the given name has already been initialized.
            if let State::Class { ref mut is_initialized } = &mut local.state {
                if *is_initialized {
                    return Err(Error::new(CLASS_ALREADY_DECLARED).with_span(node.name.span).with_tags(tags! {
                        name => node.name.identifier.clone()
                    }));
                }

                *is_initialized = true;
            }

            local.slot as u8
        };

        // NOTE: The base class is already on top of the stack from being stored in the super local.
        emit_bytecode! {
            self.assembler()?, node.span => [
                INHERIT_CLASS &node.name.identifier;
                STORE_LOCAL slot;
            ]
        }

        for associated_item in node.associated_items.iter().copied() {
            let node = self.syntax_tree.get(associated_item);

            match node {
                SyntaxNode::FnDecl(fn_decl) => {
                    let fn_decl = fn_decl.clone();
                    self.visit_fn(slot, fn_decl, class_kind)?;
                }
                SyntaxNode::OpDecl(op_decl) => {
                    let op_decl = op_decl.clone();
                    self.visit_op(slot, op_decl)?;
                }
                _ => unreachable!("Unexpected node kind encountered."),
            }
        }

        self.close_upvalues(node)?;
        self.context()?.scope_stack().pop_scope()?;

        Ok(())
    }
}

impl Compiler {
    fn visit_fn(&mut self, slot: u8, fn_decl: FnDecl, class_kind: ClassKind) -> Result<(), Error> {
        let self_param = fn_decl.args.first().filter(|arg| arg.name == *SELF.get());
        let kind = if let Some(self_param) = self_param {
            // NOTE: If the self parameter has a type annotation, return an error.
            if self_param.type_.is_some() {
                return Err(Error::new(METHOD_RECEIVER_CANNOT_HAVE_TYPE).with_span(self_param.span));
            }

            if fn_decl.name.identifier == *NEW.get() {
                FnKind::Constructor(class_kind)
            } else {
                FnKind::Method
            }
        } else {
            if fn_decl.name.identifier == *NEW.get() {
                // TODO: Propagate the span of the function's name only.
                return Err(Error::new(NEW_METHOD_MUST_HAVE_RECEIVER).with_span(fn_decl.span));
            }

            FnKind::StaticMethod
        };

        self.visit((&fn_decl, kind))?;

        emit_bytecode! {
            self.assembler()?, fn_decl.span => [
                if matches!(kind, FnKind::Constructor(_) | FnKind::Method) => [
                    STORE_METHOD &*fn_decl.name.identifier;
                    LOAD_LOCAL slot;
                ] else [
                    STORE_FIELD &*fn_decl.name.identifier;
                    POP;
                    LOAD_LOCAL slot;
                ]
            ]
        }

        Ok(())
    }

    fn visit_op(&mut self, slot: u8, op_decl: OpDecl) -> Result<(), Error> {
        let self_param = op_decl.args.first().filter(|arg| arg.name == *SELF.get());

        if let Some(self_param) = self_param {
            if self_param.type_.is_some() {
                return Err(Error::new(METHOD_RECEIVER_CANNOT_HAVE_TYPE).with_span(self_param.span));
            }
        } else {
            // TODO: Propagate the span of the operator's name only.
            return Err(Error::new(NEW_METHOD_MUST_HAVE_RECEIVER).with_span(op_decl.span));
        }

        self.visit((&op_decl, OpKind::Method))?;

        emit_bytecode! {
            self.assembler()?, op_decl.span => [
                STORE_METHOD Self::op_name(&op_decl);
                LOAD_LOCAL slot;
            ]
        }

        Ok(())
    }

    pub fn close_upvalues(&mut self, class: &ClassDecl) -> Result<(), Error> {
        let scope = self.context()?.scope_stack().top_mut()?;

        for variable in scope.variables.clone() {
            if variable.is_captured {
                self.context()?.assembler().close_upvalue(variable.slot as u8, class.span);
            }
        }

        Ok(())
    }
}
