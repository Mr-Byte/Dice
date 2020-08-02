use super::func::{Func1, Func2};
use crate::interpreter::{
    error::RuntimeError,
    object::{key::ObjectKey, operator::coalesce, reference::ObjectRef, reflection::TypeData, ObjectBase},
    symbol::common::{operators::*, types::TY_INT},
};
use maplit::hashmap;
use std::collections::HashMap;

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_NOT) => ObjectRef::new(Func1::new(not)),
        ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func2::new(eq)),
        ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func2::new(ne)),
        ObjectKey::Symbol(OP_AND) => ObjectRef::new(Func2::new(and)),
        ObjectKey::Symbol(OP_OR) => ObjectRef::new(Func2::new(or)),
        ObjectKey::Symbol(OP_COALESCE) => ObjectRef::new(Func2::from_raw(coalesce))
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_INT, Vec::new());
}

impl ObjectBase for bool {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        OPERATIONS.with(|ops| {
            ops.clone()
                .into_iter()
                .map(|(key, value)| (key, value.instance_type_data().clone()))
                .collect::<Vec<_>>()
        })
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data().clone()
    }
}

fn not(arg: &bool) -> bool {
    !arg
}

fn eq(lhs: &bool, rhs: &bool) -> bool {
    lhs == rhs
}

fn ne(lhs: &bool, rhs: &bool) -> bool {
    lhs != rhs
}

fn and(lhs: &bool, rhs: &bool) -> bool {
    *lhs && *rhs
}

fn or(lhs: &bool, rhs: &bool) -> bool {
    *lhs || *rhs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq_with_two_bools() -> Result<(), RuntimeError> {
        let lhs = ObjectRef::new(true);
        let rhs = ObjectRef::new(false);
        let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

        assert_eq!(false, *result.value::<bool>().unwrap());

        Ok(())
    }
}
