use crate::value::Value;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Array(Rc<RefCell<Vec<Value>>>);

impl Array {
    pub fn elements(&self) -> Ref<'_, Vec<Value>> {
        self.0.borrow()
    }

    pub fn elements_mut(&self) -> RefMut<'_, Vec<Value>> {
        self.0.borrow_mut()
    }
}

impl Display for Array {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .0
            .borrow()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(fmt, "[{}]", items)
    }
}

impl From<Vec<Value>> for Array {
    fn from(value: Vec<Value>) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}
