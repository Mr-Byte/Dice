use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
};
use dice_core::protocol::{class::SELF, ProtocolSymbol};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{Block, FnArg, LitIdent};

pub enum BlockKind<'args> {
    Block,
    Loop,
    Function(&'args [FnArg]),
    Method(&'args [FnArg]),
    Constructor(&'args [FnArg]),
}

impl<'args> NodeVisitor<(&Block, BlockKind<'args>)> for Compiler {
    fn visit(&mut self, (block, kind): (&Block, BlockKind<'args>)) -> Result<(), CompilerError> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);

        if let BlockKind::Function(args) | BlockKind::Method(args) | BlockKind::Constructor(args) = kind {
            // NOTE: The calling convention uses the first parameter as self in methods, but for functions it's inaccessible.
            if let BlockKind::Function(_) = kind {
                self.context()?.scope_stack().add_local(
                    "",
                    State::Local {
                        is_mutable: false,
                        is_initialized: true,
                    },
                )?;
            }

            for arg in args {
                let slot = self.context()?.scope_stack().add_local(
                    arg.name.clone(),
                    State::Local {
                        is_mutable: false,
                        is_initialized: true,
                    },
                )? as u8;

                if let Some(type_) = &arg.type_ {
                    if type_.is_nullable {
                        let null_jump;
                        emit_bytecode! {
                            // TODO: Add ASSERT_TYPE_OR_NULL
                            self.assembler()?, arg.span => [
                                LOAD_LOCAL slot;
                                PUSH_NULL;
                                EQ;
                                JUMP_IF_TRUE -> null_jump;
                                LOAD_LOCAL slot;
                                {self.visit(&LitIdent { name: type_.name.clone(), span: arg.span })?};
                                ASSERT_TYPE;
                                PATCH_JUMP <- null_jump;
                            ]
                        }
                    } else {
                        emit_bytecode! {
                            self.assembler()?, arg.span => [
                                LOAD_LOCAL slot;
                                {self.visit(&LitIdent { name: type_.name.clone(), span: arg.span })?};
                                ASSERT_TYPE;
                            ]
                        }
                    }
                }
            }
        }

        self.scan_item_decls(block)?;

        for expression in block.expressions.iter() {
            self.visit(*expression)?;
            self.assembler()?.pop(block.span);
        }

        match block.trailing_expression {
            Some(trailing_expression) => {
                self.visit(trailing_expression)?;
            }
            None => self.assembler()?.push_unit(block.span),
        }

        // NOTE: If in context of a loop, pop the last value off the stack.
        if let BlockKind::Loop = kind {
            self.assembler()?.pop(block.span);
        }

        let scope = self.context()?.scope_stack().top_mut()?;

        for variable in scope.variables.clone() {
            if variable.is_captured {
                self.context()?
                    .assembler()
                    .close_upvalue(variable.slot as u8, block.span);
            }
        }

        if let BlockKind::Function(_) = kind {
            /* NOTE: If in context of a function, implicitly return the top item on the stack.
             * If the previous instruction was a return, this will never execute.
             */
            self.assembler()?.ret(block.span)
        } else if let BlockKind::Constructor(_) = kind {
            // NOTE: If in context of a constructor, pop the last value, load self, return.
            let local_slot = self
                .context()?
                .scope_stack()
                .local(&*SELF.get())
                .expect("Methods should always have a self.")
                .slot as u8;

            emit_bytecode! {
                self.assembler()?, block.span => [
                    POP;
                    LOAD_LOCAL local_slot;
                    RET;
                ]
            }
        }

        self.context()?.scope_stack().pop_scope()?;

        Ok(())
    }
}
