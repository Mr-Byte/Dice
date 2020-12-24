use crate::module::ModuleLoader;
use dice_core::{
    error::Error,
    protocol::{
        class::NEW,
        iterator::{DONE, NEXT, VALUE},
        ProtocolSymbol,
    },
    runtime::Runtime,
    value::{NativeFn, Symbol, Value, ValueKind},
};
use std::cell::RefCell;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_array(&mut self) {
        let class = self.any_class.derive("Array");

        class.set_method(&NEW, Box::new(construct_array) as NativeFn);
        class.set_method("push", Box::new(push) as NativeFn);
        class.set_method("pop", Box::new(pop) as NativeFn);
        class.set_method("length", Box::new(length) as NativeFn);
        class.set_method("first", Box::new(first) as NativeFn);
        class.set_method("filter", Box::new(filter) as NativeFn);
        class.set_method("map", Box::new(map) as NativeFn);
        class.set_method("iter", Box::new(iter) as NativeFn);

        self.set_value_class(ValueKind::Array, class);
    }
}

fn construct_array(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [_, rest @ ..] => Ok(Value::Array(rest.to_vec().into())),
        _ => Ok(Value::Null),
    }
}

fn push(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    if let [Value::Array(arr), param, ..] = args {
        arr.push(param.clone());

        Ok(Value::Unit)
    } else {
        Ok(Value::Null)
    }
}

fn pop(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    if let [Value::Array(arr), ..] = args {
        let result = arr.pop().unwrap_or(Value::Unit);

        Ok(result)
    } else {
        Ok(Value::Null)
    }
}

fn length(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    if let [Value::Array(arr), ..] = args {
        Ok(Value::Int(arr.elements().len() as i64))
    } else {
        Ok(Value::Null)
    }
}

fn first(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
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

fn filter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
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

fn map(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
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

fn iter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Array(arr), ..] => {
            let arr = arr.clone();
            let current = RefCell::new(0);
            let value_symbol: Symbol = VALUE.get();
            let done_symbol: Symbol = DONE.get();
            let next: NativeFn = Box::new(move |runtime, _| {
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
            iterator.set_field(&NEXT, Value::with_native_fn(next));

            Ok(Value::Object(iterator))
        }
        _ => Ok(Value::Null),
    }
}
