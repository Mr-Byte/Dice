use crate::value::Value;
use gc::{Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};

#[derive(Debug, Trace, Finalize)]
pub enum UpvalueState {
    Open(usize),
    Closed(Value),
}

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Upvalue(Gc<GcCell<UpvalueState>>);

impl Upvalue {
    pub fn new_open(slot: usize) -> Self {
        Self(Gc::new(GcCell::new(UpvalueState::Open(slot))))
    }

    pub fn close(&self, value: Value) {
        *self.0.borrow_mut() = UpvalueState::Closed(value);
    }

    pub fn state_mut(&self) -> GcCellRefMut<'_, UpvalueState> {
        self.0.borrow_mut()
    }

    pub fn state(&self) -> GcCellRef<'_, UpvalueState> {
        self.0.borrow()
    }
}
