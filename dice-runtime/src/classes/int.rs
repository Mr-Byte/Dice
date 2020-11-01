use crate::module::ModuleLoader;
use dice_core::{
    protocol::class::NEW,
    runtime::Runtime,
    value::{NativeFn, Value, ValueKind},
};
use dice_error::runtime_error::RuntimeError;
use std::rc::Rc;

impl<L> crate::Runtime<L>
where
    L: ModuleLoader,
{
    pub(super) fn register_int(&mut self) {
        let class = self.any_class.derive("Int");

        class.set_method(&NEW, Rc::new(construct_int) as NativeFn);
        class.set_method("abs", bind_i64_ret_i64(i64::abs));
        class.set_method("pow", Rc::new(pow) as NativeFn);
        class.set_method("is_positive", bind_i64_ret_bool(i64::is_positive));
        class.set_method("is_negative", bind_i64_ret_bool(i64::is_negative));
        class.set_method("min", bind_i64_i64_ret_i64(i64::min));
        class.set_method("max", bind_i64_i64_ret_i64(i64::max));

        class.set_field("MAX", Value::Int(i64::MAX));
        class.set_field("MIN", Value::Int(i64::MIN));
        class.set_field("I32_MAX", Value::Int(i32::MAX as i64));
        class.set_field("I32_MIN", Value::Int(i32::MIN as i64));
        class.set_field("U32_MAX", Value::Int(u32::MAX as i64));
        class.set_field("U32_MIN", Value::Int(u32::MIN as i64));
        class.set_field("I16_MAX", Value::Int(i16::MAX as i64));
        class.set_field("I16_MIN", Value::Int(i16::MIN as i64));
        class.set_field("U16_MAX", Value::Int(u16::MAX as i64));
        class.set_field("U16_MIN", Value::Int(u16::MIN as i64));
        class.set_field("I8_MAX", Value::Int(i8::MAX as i64));
        class.set_field("I8_MIN", Value::Int(i8::MIN as i64));
        class.set_field("U8_MAX", Value::Int(u8::MAX as i64));
        class.set_field("U8_MIN", Value::Int(u8::MIN as i64));

        self.set_value_class(ValueKind::Int, class);
    }
}

fn construct_int(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    match args {
        [_, param, ..] => match param {
            value @ Value::Int(_) => Ok(value.clone()),
            Value::Float(value) => Ok(Value::Int(*value as i64)),
            Value::String(string) => Ok(string.parse::<i64>().map_or_else(|_| Value::Null, Value::Int)),
            _ => Ok(Value::Null),
        },
        _ => Ok(Value::Null),
    }
}

fn bind_i64_ret_i64(function: impl Fn(i64) -> i64 + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(this), ..] => Ok(Value::Int(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_i64_ret_bool(function: impl Fn(i64) -> bool + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(this), ..] => Ok(Value::Bool(function(*this))),
        _ => Ok(Value::Null),
    })
}

fn bind_i64_i64_ret_i64(function: impl Fn(i64, i64) -> i64 + 'static) -> NativeFn {
    Rc::new(move |_: &mut dyn Runtime, args: &[Value]| match args {
        [Value::Int(first), Value::Int(second), ..] => Ok(Value::Int(function(*first, *second))),
        _ => Ok(Value::Null),
    })
}

fn pow(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [Value::Int(this), Value::Int(exp), ..] = args {
        Ok(Value::Int(this.pow(*exp as u32)))
    } else {
        Ok(Value::Null)
    }
}
