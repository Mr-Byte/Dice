use crate::module::ModuleLoader;
use dice_core::{
    protocol::{
        class::NEW,
        iterator::{DONE, NEXT, VALUE},
    },
    runtime::Runtime,
    value::{NativeFn, Symbol, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::{cell::RefCell, rc::Rc};

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_array(&mut self) {
        let class = self.object_class.derive("Array");

        class.set_method(NEW, Rc::new(construct_array) as NativeFn);
        class.set_method("push", Rc::new(push) as NativeFn);
        class.set_method("pop", Rc::new(pop) as NativeFn);
        class.set_method("length", Rc::new(length) as NativeFn);
        class.set_method("first", Rc::new(first) as NativeFn);
        class.set_method("filter", Rc::new(filter) as NativeFn);
        class.set_method("map", Rc::new(map) as NativeFn);
        class.set_method("iter", Rc::new(iter) as NativeFn);

        self.known_types.insert(ValueKind::Array, class.clone());
        self.globals.insert(class.name(), Value::Class(class.clone()));
    }
}

fn construct_array(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, rest @ ..] = args {
        let arr = rest.to_vec();

        Ok(Value::Array(arr.into()))
    } else {
        Ok(Value::Null)
    }
}

fn push(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), param, ..] = args {
        arr.elements_mut().push(param.clone());

        Ok(Value::Unit)
    } else {
        Ok(Value::Null)
    }
}

fn pop(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), ..] = args {
        let result = arr.elements_mut().pop().unwrap_or_default();

        Ok(result)
    } else {
        Ok(Value::Null)
    }
}

fn length(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Array(arr), ..] = args {
        Ok(Value::Int(arr.elements().len() as i64))
    } else {
        Ok(Value::Null)
    }
}

fn first(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), predicate, ..] => Ok(arr
            .elements()
            .iter()
            .find(|value| {
                runtime
                    .call_function(predicate.clone(), &[(*value).clone()])
                    .ok()
                    .and_then(|value| value.as_bool().ok())
                    .unwrap_or(false)
            })
            .cloned()
            .unwrap_or(Value::Null)),
        [Value::Array(arr), ..] => Ok(arr.elements().first().cloned().unwrap_or(Value::Null)),
        _ => Ok(Value::Null),
    }
}

fn filter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), predicate, ..] => Ok(Value::Array(
            arr.elements()
                .iter()
                .filter(|value| {
                    runtime
                        .call_function(predicate.clone(), &[(*value).clone()])
                        .ok()
                        .and_then(|value| value.as_bool().ok())
                        .unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
                .into(),
        )),
        _ => Ok(Value::Null),
    }
}

fn map(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), selector, ..] => Ok(Value::Array(
            arr.elements()
                .iter()
                .map(|value| {
                    runtime
                        .call_function(selector.clone(), &[(*value).clone()])
                        .ok()
                        .unwrap_or(Value::Null)
                })
                .collect::<Vec<_>>()
                .into(),
        )),
        _ => Ok(Value::Null),
    }
}

fn iter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [Value::Array(arr), ..] => {
            let arr = arr.clone();
            let current = RefCell::new(0);
            let value_symbol: Symbol = VALUE.into();
            let done_symbol: Symbol = DONE.into();
            let next: NativeFn = Rc::new(move |runtime, _| {
                let result = runtime.new_object()?;

                if *current.borrow() < arr.elements().len() {
                    result.set_field(value_symbol.clone(), arr.elements()[*current.borrow()].clone());
                    result.set_field(done_symbol.clone(), Value::Bool(false));
                    *current.borrow_mut() += 1;
                } else {
                    result.set_field(done_symbol.clone(), Value::Bool(true));
                }

                Ok(Value::Object(result))
            });

            let iterator = runtime.new_object()?;
            iterator.set_field(NEXT, Value::with_native_fn(next));

            Ok(Value::Object(iterator))
        }
        _ => Ok(Value::Null),
    }
}
