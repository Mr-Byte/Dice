use crate::value::Value;
use static_assertions::_core::fmt::Formatter;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Object {
    inner: Rc<ObjectInner>,
}

impl Object {
    pub fn new(type_id: u64) -> Self {
        Self {
            inner: Rc::new(ObjectInner {
                fields: Default::default(),
                type_id,
            }),
        }
    }
}

impl Deref for Object {
    type Target = ObjectInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: Should this print more useful info?
        write!(f, "Object")
    }
}

#[derive(Debug)]
pub struct ObjectInner {
    fields: RefCell<HashMap<String, Value>>,
    type_id: u64,
}

impl ObjectInner {
    pub fn fields(&self) -> Ref<'_, HashMap<String, Value>> {
        self.fields.borrow()
    }

    pub fn fields_mut(&self) -> RefMut<'_, HashMap<String, Value>> {
        self.fields.borrow_mut()
    }

    pub fn type_id(&self) -> u64 {
        self.type_id
    }
}
