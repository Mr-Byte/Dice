use crate::value::{Object, Value};
use std::ops::Deref;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    inner: Rc<ArrayInner>,
}

impl Array {
    pub fn elements(&self) -> Ref<'_, [Value]> {
        Ref::map(self.array.borrow(), |array| array.as_slice())
    }

    pub fn elements_mut(&self) -> RefMut<'_, [Value]> {
        RefMut::map(self.array.borrow_mut(), |array| array.as_mut_slice())
    }

    pub fn push(&self, value: Value) {
        self.array.borrow_mut().push(value)
    }

    pub fn pop(&self) -> Option<Value> {
        self.array.borrow_mut().pop()
    }
}

impl Display for Array {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .array
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
        Self {
            inner: Rc::new(ArrayInner {
                array: RefCell::new(value),
                object: Object::new(None),
            }),
        }
    }
}

impl Deref for Array {
    type Target = ArrayInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInner {
    array: RefCell<Vec<Value>>,
    object: Object,
}

impl Deref for ArrayInner {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
