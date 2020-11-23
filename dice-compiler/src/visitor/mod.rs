mod decl_class;
mod decl_export;
mod decl_fn;
mod decl_import;
mod decl_op;
mod decl_var;
mod expr_assignment;
mod expr_binary_op;
mod expr_block;
mod expr_break;
mod expr_continue;
mod expr_error_propagate;
mod expr_field_access;
mod expr_fn_call;
mod expr_for;
mod expr_if;
mod expr_index;
mod expr_is;
mod expr_loop;
mod expr_null_propagate;
mod expr_range_loop;
mod expr_return;
mod expr_super_access;
mod expr_super_call;
mod expr_unary_op;
mod expr_while;
mod literal_anonymous_fn;
mod literal_bool;
mod literal_float;
mod literal_int;
mod literal_list;
mod literal_null;
mod literal_object;
mod literal_string;
mod literal_unit;
mod literal_variable;
mod syntax_node;

use dice_error::compiler_error::CompilerError;

pub use expr_block::{BlockKind, FunctionBlockKind};

pub(super) trait NodeVisitor<T> {
    fn visit(&mut self, node: T) -> Result<(), CompilerError>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ClassKind {
    Base,
    Derived,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FnKind {
    Function,
    Method,
    StaticMethod,
    Constructor(ClassKind),
}
