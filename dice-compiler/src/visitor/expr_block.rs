use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    scope_stack::{ScopeKind, State},
};
use dice_core::protocol::{class::SELF, ProtocolSymbol};
use dice_error::compiler_error::CompilerError;
use dice_syntax::{Block, FnArg, LitIdent};

pub enum BlockKind {
    Block,
    Loop,
}

pub enum FunctionBlockKind<'args> {
    Function(&'args [FnArg]),
    Method(&'args [FnArg]),
    Constructor(&'args [FnArg]),
}

impl<'args> FunctionBlockKind<'args> {
    fn args(&self) -> &'args [FnArg] {
        match self {
            FunctionBlockKind::Function(args)
            | FunctionBlockKind::Method(args)
            | FunctionBlockKind::Constructor(args) => *args,
        }
    }
}

impl NodeVisitor<(&Block, BlockKind)> for Compiler {
    fn visit(&mut self, (block, kind): (&Block, BlockKind)) -> Result<(), CompilerError> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);
        self.scan_item_decls(block)?;
        self.visit_expressions(block)?;

        // NOTE: If in context of a loop, pop the last value off the stack.
        if let BlockKind::Loop = kind {
            self.assembler()?.pop(block.span);
        }

        self.visit_close_upvalues(block)?;
        self.context()?.scope_stack().pop_scope()?;

        Ok(())
    }
}

impl<'args> NodeVisitor<(&Block, FunctionBlockKind<'args>)> for Compiler {
    fn visit(&mut self, (block, kind): (&Block, FunctionBlockKind<'args>)) -> Result<(), CompilerError> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);
        self.visit_args(&kind, kind.args())?;
        self.scan_item_decls(block)?;
        self.visit_expressions(block)?;

        if let FunctionBlockKind::Function(_) = kind {
            /* NOTE: If in context of a function, implicitly return the top item on the stack.
             * If the previous instruction was a return, this will never execute.
             */
            self.assembler()?.ret(block.span)
        } else if let FunctionBlockKind::Constructor(_) = kind {
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

impl Compiler {
    fn visit_args(&mut self, kind: &FunctionBlockKind, args: &[FnArg]) -> Result<(), CompilerError> {
        // NOTE: The calling convention uses the first parameter as self in methods, but for functions it's inaccessible.
        if let FunctionBlockKind::Function(_) = kind {
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
                emit_bytecode! {
                    self.assembler()?, arg.span => [
                        {self.visit(&LitIdent { name: type_.name.clone(), span: arg.span })?};
                        if type_.is_nullable => [
                            ASSERT_TYPE_OR_NULL_FOR_LOCAL slot;
                        ] else [
                            ASSERT_TYPE_FOR_LOCAL slot;
                        ]
                    ]
                }
            }
        }

        Ok(())
    }

    fn visit_close_upvalues(&mut self, block: &Block) -> Result<(), CompilerError> {
        let scope = self.context()?.scope_stack().top_mut()?;

        for variable in scope.variables.clone() {
            if variable.is_captured {
                self.context()?
                    .assembler()
                    .close_upvalue(variable.slot as u8, block.span);
            }
        }

        Ok(())
    }

    fn visit_expressions(&mut self, block: &Block) -> Result<(), CompilerError> {
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

        Ok(())
    }
}
