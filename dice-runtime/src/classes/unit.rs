use crate::module::ModuleLoader;
use dice_core::value::ValueKind;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_unit(&mut self) {
        let class = self.any_class.derive("Unit");

        self.set_value_class(ValueKind::Unit, class);
    }
}
