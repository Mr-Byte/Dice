use std::cell::{Ref, RefMut};

use gc_arena::{lock::RefLock, Collect, Gc};

use crate::{runtime::RuntimeContext, value::Value};

#[derive(Collect)]
#[collect(no_drop)]
pub enum UpvalueState<'gc> {
    Open(usize),
    Closed(Value<'gc>),
}

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct Upvalue<'gc>(Gc<'gc, RefLock<UpvalueState<'gc>>>);

impl<'gc> Upvalue<'gc> {
    pub fn new_open(ctx: &RuntimeContext<'gc>, slot: usize) -> Self {
        Self(Gc::new(ctx.mutation, RefLock::new(UpvalueState::Open(slot))))
    }

    pub fn close(&self, ctx: &RuntimeContext<'gc>, value: Value<'gc>) {
        *self.0.borrow_mut(ctx.mutation) = UpvalueState::Closed(value);
    }

    pub fn state_mut(&self, ctx: &RuntimeContext<'gc>) -> RefMut<'gc, UpvalueState> {
        self.0.borrow_mut(ctx.mutation)
    }

    pub fn state(&self) -> Ref<'gc, UpvalueState> {
        self.0.borrow()
    }
}
