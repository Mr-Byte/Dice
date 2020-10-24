mod call_frame;

pub use call_frame::*;

use dice_core::value::Value;
use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
};

// NOTE: Allocate 1MB of stack space, this is 65,536 values when sizeof(Value) == 16
const MAX_STACK_SIZE: usize = (1024 * 1024) / std::mem::size_of::<Value>();

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
    stack_ptr: usize,
}

// TODO: Enforce stack overflows and underflows.
impl Stack {
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.values[self.stack_ptr] = value;
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
    }

    pub fn push_slice(&mut self, values: &[Value]) {
        // TODO: Replace this with a more efficient multi-push.
        for value in values {
            self.push(value.clone());
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
        std::mem::replace(&mut self.values[self.stack_ptr], Value::Null)
    }

    pub fn pop_count(&mut self, count: usize) -> Vec<Value> {
        let mut result = vec![Value::Null; count];
        let items_to_pop = &mut self.values[self.stack_ptr.wrapping_sub(count)..self.stack_ptr];
        self.stack_ptr = self.stack_ptr.wrapping_sub(count);

        for index in (0..items_to_pop.len()).rev() {
            std::mem::swap(&mut items_to_pop[index], &mut result[index])
        }

        result
    }

    pub fn reserve_slots(&mut self, count: usize) -> CallFrame {
        let start = self.stack_ptr;
        let new_stack_ptr = self.stack_ptr.wrapping_add(count);

        self.stack_ptr = new_stack_ptr;
        debug_assert!(self.stack_ptr < MAX_STACK_SIZE, "Stack Overflowed");

        CallFrame::new(start, new_stack_ptr)
    }

    // TODO: Should this be a release call frame?
    pub fn release_slots(&mut self, count: usize) {
        let new_stack_ptr = self.stack_ptr.wrapping_sub(count);
        for value in &mut self.values[new_stack_ptr..self.stack_ptr] {
            *value = Value::Null;
        }

        self.stack_ptr = new_stack_ptr;

        // NOTE: If the stack ptr is greater than the stack size, the stack ptr underflowed.
        debug_assert!(self.stack_ptr < MAX_STACK_SIZE, "Stack Underflowed")
    }

    // NOTE: Returns the value offset from the top of the stack.
    #[inline]
    pub fn peek_mut(&mut self, offset: usize) -> &mut Value {
        &mut self.values[self.stack_ptr.wrapping_sub(offset).wrapping_sub(1)]
    }

    #[inline]
    pub fn peek(&self, offset: usize) -> &Value {
        &self.values[self.stack_ptr.wrapping_sub(offset).wrapping_sub(1)]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stack_ptr
    }
}

impl Index<usize> for Stack {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl Index<CallFrame> for Stack {
    type Output = [Value];

    fn index(&self, index: CallFrame) -> &Self::Output {
        &self.values[index.range()]
    }
}

impl IndexMut<CallFrame> for Stack {
    fn index_mut(&mut self, index: CallFrame) -> &mut Self::Output {
        &mut self.values[index.range()]
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            values: vec![Value::Null; MAX_STACK_SIZE],
            stack_ptr: 0,
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stack = [")?;

        for (index, value) in self.values.iter().enumerate() {
            if index >= self.stack_ptr {
                break;
            }

            writeln!(f, "\t[{:#06X}] = {},", index, value)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}
