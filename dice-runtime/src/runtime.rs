use std::{
    collections::{HashMap, VecDeque},
    hash::BuildHasherDefault,
};

use ahash::AHasher;
use gc_arena::{Arena, Collect, Mutation, Rootable};

use dice_bytecode::Bytecode;
use dice_core::error::Error;

use crate::upvalue::Upvalue;
use crate::value::{Class, SymbolInterner, Value, ValueKind, ValueMap};
use crate::{
    module::{file_loader::FileModuleLoader, ModuleLoader},
    stack::Stack,
};

pub struct RuntimeContext<'gc> {
    pub mutation: &'gc Mutation<'gc>,
    pub interner: &'gc SymbolInterner,
    pub state: &'gc State<'gc>,
}

pub struct Runtime<L = FileModuleLoader>
where
    L: ModuleLoader + 'static,
{
    pub(crate) state: Arena<Rootable![State<'_, L>]>,
    pub(crate) interner: SymbolInterner,
}

#[derive(Collect)]
#[collect(no_drop)]
pub(crate) struct State<'gc, L = FileModuleLoader> {
    pub(super) stack: Stack<'gc>,
    pub(crate) open_upvalues: VecDeque<Upvalue<'gc>>,
    pub(crate) globals: ValueMap<'gc>,
    pub(crate) loaded_modules: ValueMap<'gc>,
    #[collect(require_static)]
    pub(crate) module_loader: L,
    // pub(crate) any_class: Class<'gc>,
    // pub(crate) module_class: Class<'gc>,
    pub(crate) value_class_mapping: HashMap<ValueKind, Class<'gc>, BuildHasherDefault<AHasher>>,
}

impl<'gc, L> State<'gc, L>
where
    L: ModuleLoader,
{
    pub fn new(mutation: &Mutation<'gc>) -> Self {
        // let any_class = Self::new_any_class(mutation);
        // let module_class = Self::new_module_class(&any_class);
        // let mut globals: ValueMap = ValueMap::default();
        // globals.insert(any_class.name(), Value::Class(any_class.clone()));
        // globals.insert(module_class.name(), Value::Class(module_class.clone()));

        Self {
            stack: Default::default(),
            open_upvalues: Default::default(),
            loaded_modules: Default::default(),
            module_loader: Default::default(),
            value_class_mapping: Default::default(),
            globals: Default::default(),
            // any_class,
            // module_class,
        }
    }
}

impl<'gc, L> Default for Runtime<L>
where
    L: ModuleLoader,
{
    fn default() -> Self {
        let mut runtime = Self {
            state: Arena::<Rootable![State<'_, L>]>::new(State::new),
            interner: Default::default(),
        };

        // runtime.register_known_types();
        runtime
    }
}

impl<L> Runtime<L>
where
    L: ModuleLoader + Default,
{
    pub fn run(&mut self, bytecode: Bytecode) -> Result<Value, Error> {
        self.state.mutate_root(|ctx, mut state| {
            let stack_frame = state.stack.reserve_slots(bytecode.slot_count());
            let result = self.execute(&bytecode, stack_frame, None)?;

            state.stack.release_stack_frame(stack_frame);

            Ok(result)
        })
    }

    pub(super) fn run_module(&mut self, bytecode: Bytecode, export: Value) -> Result<Value, Error> {
        self.state.mutate_root(|ctx, mut state| {
            let stack_frame = state.stack.reserve_slots(bytecode.slot_count());
            state.stack[stack_frame.start()] = export;
            let result = self.execute(&bytecode, stack_frame, None)?;
            state.stack.release_stack_frame(stack_frame);

            Ok(result)
        })
    }

    // pub(super) fn set_value_class(&mut self, value_kind: ValueKind, class: Class) {
    //     self.state.mutate(|_, state| {
    //         state.value_class_mapping.insert(value_kind, class.clone());
    //         state.globals.insert(class.name(), Value::Class(class));
    //     });
    // }
}

// impl<L> dice_core::runtime::Runtime for Runtime<L>
// where
//     L: ModuleLoader,
// {
//     fn new_module(&mut self, name: &str) -> Result<Object, Error> {
//         let module = Object::new(None);

//         if self
//             .loaded_modules
//             .insert(name.into(), Value::Object(module.clone()))
//             .is_some()
//         {
//             return Err(Error::new(MODULE_ALREADY_EXISTS).with_tags(tags! {
//                 name => name.to_string()
//             }));
//         }

//         Ok(module)
//     }

//     fn new_class(&mut self, name: &str) -> Result<Class, Error> {
//         let class = Class::with_base(name.into(), self.any_class.clone());

//         Ok(class)
//     }

//     fn new_object(&mut self) -> Result<Object, Error> {
//         let object = Object::new(self.any_class.clone());

//         Ok(object)
//     }

//     fn load_prelude(&mut self, path: &str) -> Result<(), Error> {
//         let module = self.module_loader.load_module(path.into())?;
//         let prelude = Value::Object(Object::new(self.module_class.clone()));
//         // NOTE: Add the loaded prelude module as a registered module.
//         self.loaded_modules.insert(module.id.clone(), prelude.clone());

//         let prelude = self.run_module(module.bytecode, prelude)?;

//         for (name, value) in &*prelude.as_object()?.fields() {
//             self.globals.entry(name.clone()).or_insert_with(|| value.clone());
//         }

//         Ok(())
//     }

//     fn add_global(&mut self, name: &str, value: Value) -> Result<(), Error> {
//         if self.globals.insert(name.into(), value).is_some() {
//             return Err(Error::new(GLOBAL_ALREADY_EXISTS).with_tags(tags! {
//                 name => name.to_string()
//             }));
//         }

//         Ok(())
//     }

//     fn call_function(&mut self, target: Value, args: &[Value]) -> Result<Value, Error> {
//         let arg_count = args.len();
//         self.stack.push(target);
//         self.stack.push_multiple(args);
//         self.call_fn(arg_count)?;

//         Ok(self.stack.pop())
//     }

//     fn any_class(&self) -> Result<Class, Error> {
//         Ok(self.any_class.clone())
//     }

//     fn class_of(&self, value: &Value) -> Result<Class, Error> {
//         let result = value
//             .as_object()
//             .ok()
//             .and_then(|object| object.class())
//             .or_else(|| self.value_class_mapping.get(&value.kind()).cloned())
//             .unwrap_or_else(|| self.any_class.clone());

//         Ok(result)
//     }

//     fn is_value_of_type(&self, value: &Value, class: &Class) -> Result<bool, Error> {
//         value
//             .as_object()
//             .ok()
//             .and_then(|object| object.class())
//             .or_else(|| self.value_class_mapping.get(&value.kind()).cloned())
//             .map_or(Ok(false), |instance_class| Ok(instance_class.is_class(&class)))
//     }
// }
