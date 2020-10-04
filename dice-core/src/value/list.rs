use crate::value::Value;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct List(Rc<RefCell<Vec<Value>>>);

impl List {
    pub fn elements(&self) -> Ref<'_, [Value]> {
        Ref::map(self.0.borrow(), |elements| elements.as_slice())
    }

    pub fn elements_mut(&self) -> RefMut<'_, [Value]> {
        RefMut::map(self.0.borrow_mut(), |elements| elements.as_mut_slice())
    }
}

impl Display for List {
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

impl From<Vec<Value>> for List {
    fn from(value: Vec<Value>) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}
