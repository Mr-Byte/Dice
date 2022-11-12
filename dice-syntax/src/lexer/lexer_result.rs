use std::{cell::RefCell, rc::Rc};

use dice_core::error::Error;

#[derive(Default, Clone)]
pub struct LexerResult(Rc<RefCell<Option<Error>>>);

impl LexerResult {
    pub fn error(&self) -> Option<Error> {
        self.0.borrow_mut().take()
    }

    pub fn set_error(&self, error: Error) {
        *self.0.borrow_mut() = Some(error);
    }
}
