use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
    ops::Deref,
};

use ahash::AHasher;
use gc_arena::{Collect, Gc, lock::RefLock};

use dice_core::protocol::class::NEW;

use crate::{
    runtime::RuntimeContext,
    value::{Object, symbol::Symbol, Value, ValueKind, ValueMap},
};
use crate::type_id::TypeId;

#[derive(Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub struct Class<'gc> {
    inner: Gc<'gc, ClassInner<'gc>>,
}

impl<'gc> Class<'gc> {
    pub fn new(ctx: &RuntimeContext<'gc>, name: Symbol) -> Self {
        let instance_type_id = TypeId::new();
        let mut type_ids: HashSet<_, _> = Default::default();
        type_ids.insert(instance_type_id);

        let inner = ClassInner {
            instance_type_id,
            type_ids,
            methods: Gc::new(ctx.mutation, RefLock::new(HashMap::default())),
            object: Object::new(ctx, None),
            name,
            base: None,
        };

        Self {
            inner: Gc::new(ctx.mutation, inner),
        }
    }

    pub fn with_base(ctx: &RuntimeContext<'gc>, name: Symbol, base: Class<'gc>) -> Self {
        let methods = base
            .inner
            .methods
            .borrow()
            .iter()
            .filter(|(name, _)| ctx.interner.resolve(**name) != Some(NEW))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<HashMap<_, _, _>>();
        let instance_type_id = TypeId::new();
        let mut type_ids: HashSet<_, _> = base.inner.type_ids.clone();
        type_ids.insert(instance_type_id);

        let inner = ClassInner {
            instance_type_id,
            type_ids,
            name,
            methods: Gc::new(ctx.mutation, RefLock::new(methods)),
            object: base.inner.object.deep_clone(ctx),
            base: Some(base),
        };

        Self {
            inner: Gc::new(ctx.mutation, inner),
        }
    }

    pub fn derive(&self, ctx: &RuntimeContext<'gc>, name: impl Into<Symbol>) -> Self {
        Self::with_base(ctx, name.into(), self.clone())
    }

    pub fn is_class(&self, class: &Class) -> bool {
        let type_id = class.instance_type_id();

        self.inner.type_ids.contains(&type_id)
    }

    pub fn name(&self) -> Symbol {
        self.inner.name.clone()
    }

    pub fn instance_type_id(&self) -> TypeId {
        self.inner.instance_type_id
    }

    pub fn method(&self, name: impl Into<Symbol>) -> Option<Value<'gc>> {
        let name = name.into();
        self.inner.methods.borrow().get(&name).cloned()
    }

    pub fn set_method(&self, ctx: RuntimeContext<'gc>, name: impl Into<Symbol>, method: impl Into<Value<'gc>>) {
        let method = method.into();

        if method.kind() != ValueKind::Function {
            panic!("Provided value is not a function.");
        }

        self.inner.methods.borrow_mut(ctx.mutation).insert(name.into(), method);
    }

    pub fn methods(&self) -> Vec<(Symbol, Value<'gc>)> {
        // TODO: Make this handle multiple, conflicting methods when traits are added.
        self.inner
            .methods
            .borrow()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>()
    }

    pub fn base(&self) -> Option<Class<'gc>> {
        self.inner.base.clone()
    }
}

#[derive(Collect)]
#[collect(no_drop)]
struct ClassInner<'gc> {
    name: Symbol,
    methods: Gc<'gc, RefLock<ValueMap<'gc>>>,
    object: Object<'gc>,
    #[collect(require_static)]
    instance_type_id: TypeId,
    #[collect(require_static)]
    type_ids: HashSet<TypeId, BuildHasherDefault<AHasher>>,
    base: Option<Class<'gc>>,
}

impl<'gc> Deref for Class<'gc> {
    type Target = Object<'gc>;

    fn deref(&self) -> &Self::Target {
        &self.inner.object
    }
}

impl<'gc> PartialEq for ClassInner<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.instance_type_id == other.instance_type_id
    }
}

impl Eq for ClassInner<'_> {}
