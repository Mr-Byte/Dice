use crate::value::Object;
use gc::{Finalize, Trace};

#[derive(Default, Clone, Debug, Trace, Finalize)]
pub struct Module {
    module_object: Object,
}

impl Module {
    pub fn object(&self) -> Object {
        self.module_object.clone()
    }
}
