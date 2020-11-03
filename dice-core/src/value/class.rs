use crate::value::ValueKind;
use crate::{
    type_id::TypeId,
    value::{symbol::Symbol, Object, Value, ValueMap},
};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    ops::Deref,
    rc::Rc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class {
    inner: Rc<ClassInner>,
}

impl Class {
    pub fn new(name: Symbol) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            base: Default::default(),
            methods: Default::default(),
            object: Object::new(None),
            name,
        };

        Self { inner: inner.into() }
    }

    pub fn with_base(name: Symbol, base: Class) -> Self {
        let inner = ClassInner {
            instance_type_id: TypeId::new(),
            methods: Default::default(),
            object: Object::new(None),
            base: Some(base),
            name,
        };

        Self { inner: inner.into() }
    }

    pub fn derive(&self, name: impl Into<Symbol>) -> Self {
        Self::with_base(name.into(), self.clone())
    }

    pub fn is_class(&self, class: &Class) -> bool {
        let type_id = class.instance_type_id();

        self.instance_type_id() == type_id || self.base.as_ref().map_or_else(|| false, |base| base.is_class(class))
    }

    pub fn relates_to_class_as(&self, other: &Class) -> ClassOrder {
        if self.instance_type_id() == other.instance_type_id() {
            ClassOrder::Same
        } else if self.is_class(other) {
            ClassOrder::Derived
        } else if other.is_class(self) {
            ClassOrder::Base
        } else {
            ClassOrder::None
        }
    }

    pub fn name(&self) -> Symbol {
        self.name.clone()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.instance_type_id
    }

    pub fn method(&self, name: impl Into<Symbol>) -> Option<Value> {
        let name = name.into();
        self.methods
            .borrow()
            .get(&name)
            .cloned()
            .or_else(|| self.base.as_ref().and_then(|base| base.method(name)))
    }

    pub fn set_method(&self, name: impl Into<Symbol>, method: impl Into<Value>) {
        let method = method.into();

        if method.kind() != ValueKind::Function {
            // TODO: Return error.
        }

        self.methods.borrow_mut().insert(name.into(), method);
    }

    pub fn methods(&self) -> Vec<(Symbol, Value)> {
        // TODO: Make this handle multiple, conflicting methods when traits are added.
        self.methods
            .borrow()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .chain(self.base.iter().flat_map(|base| base.methods()))
            .collect::<Vec<_>>()
    }

    pub fn base(&self) -> Option<Class> {
        self.base.clone()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class{{{}}}", self.name)
    }
}

#[derive(Debug)]
pub struct ClassInner {
    name: Symbol,
    base: Option<Class>,
    methods: RefCell<ValueMap>,
    object: Object,
    instance_type_id: TypeId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassOrder {
    None,
    Derived,
    Same,
    Base,
}

impl Deref for Class {
    type Target = ClassInner;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl Deref for ClassInner {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl PartialEq for ClassInner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_id() == other.type_id()
    }
}

impl Eq for ClassInner {}

#[test]
fn test_class_ordering() {
    let base = Class::new("base".into());
    let derived = Class::with_base("derived".into(), base.clone());
    let unrelated = Class::with_base("unrelated".into(), base.clone());

    assert_eq!(base.relates_to_class_as(&derived), ClassOrder::Base);
    assert_eq!(derived.relates_to_class_as(&base), ClassOrder::Derived);
    assert_eq!(base.relates_to_class_as(&base), ClassOrder::Same);
    assert_eq!(derived.relates_to_class_as(&derived), ClassOrder::Same);
    assert_eq!(unrelated.relates_to_class_as(&derived), ClassOrder::None);
    assert_eq!(derived.relates_to_class_as(&unrelated), ClassOrder::None);
}
