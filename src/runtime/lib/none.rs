use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use gc::{Finalize, Trace};
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Trace, Finalize)]
pub struct None;

impl TypeInstance for None {}

impl Display for None {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}

decl_type! {
    impl TypeNone for None as "None";

    fn op_eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>()?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_some()))
    }

    fn op_neq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        lhs.try_value::<None>()?;
        let rhs = rhs.value::<None>();

        Ok(Value::new(rhs.is_none()))
    }
}
