use crate::value::Class;
use std::rc::Rc;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct MethodPattern {
    inner: Rc<[TypePattern]>,
}

#[derive(Debug, Clone)]
pub struct TypePattern {
    type_: Option<Type>,
    is_nullable: bool,
}

#[derive(Debug, Clone)]
pub enum Type {
    Class(Class),
}

pub enum MethodOrder {
    None,
    Before,
    After,
    Equal,
}
