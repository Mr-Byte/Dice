use crate::{module::ModuleLoader, runtime::Runtime};
use dice_core::{
    error::Error,
    protocol::{class::NEW, ProtocolSymbol},
    upvalue::{Upvalue, UpvalueState},
    value::{Class, FnBound, FnNative, FnScript, Object, Symbol, Value, ValueKind},
};

impl<L: ModuleLoader> Runtime<L> {
    pub(super) fn find_open_upvalue(&self, offset: usize) -> Option<(usize, Upvalue)> {
        let mut found_upvalue = None;

        for (index, upvalue) in self.open_upvalues.iter().enumerate() {
            if let UpvalueState::Open(upvalue_offset) = &*upvalue.state() {
                if *upvalue_offset == offset {
                    found_upvalue = Some((index, upvalue.clone()));
                }
            }
        }

        found_upvalue
    }

    pub(super) fn call_binary_op(&mut self, operator: impl Into<Symbol>, rhs: Value) -> Result<(), Error> {
        fn call_binary_op<L: ModuleLoader>(runtime: &mut Runtime<L>, operator: Symbol, rhs: Value) -> Result<(), Error> {
            let lhs = runtime.stack.pop();
            let method = runtime.get_field(&operator, lhs.clone())?;

            if method != Value::Null {
                runtime.stack.push(method);
                runtime.stack.push(rhs);
                runtime.call_fn(1)?;
            } else {
                let value = runtime.globals.get(&operator).ok_or_else(|| todo!("No global operator defined."))?;

                runtime.stack.push(value.clone());
                runtime.stack.push(lhs);
                runtime.stack.push(rhs);
                runtime.call_fn(2)?;
            }

            Ok(())
        }

        call_binary_op(self, operator.into(), rhs)
    }

    pub(super) fn call_unary_op(&mut self, operator: impl Into<Symbol>) -> Result<(), Error> {
        fn call_unary_op<L: ModuleLoader>(runtime: &mut Runtime<L>, operator: Symbol) -> Result<(), Error> {
            let operand = runtime.stack.pop();
            let method = runtime.get_field(&operator, operand.clone())?;

            if method != Value::Null {
                runtime.stack.push(method);
                runtime.call_fn(0)?;
            } else {
                let value = runtime.globals.get(&operator).ok_or_else(|| todo!("No global operator defined."))?;

                runtime.stack.push(value.clone());
                runtime.stack.push(operand);
                runtime.call_fn(1)?;
            }

            Ok(())
        }

        call_unary_op(self, operator.into())
    }

    pub(super) fn get_field(&self, key: &Symbol, value: Value) -> Result<Value, Error> {
        if value.kind() == ValueKind::Object || value.kind() == ValueKind::Class || value.kind() == ValueKind::Array {
            let object = value.as_object()?;
            let fields = object.fields();
            if let Some(field) = fields.get(&key) {
                return Ok(field.clone());
            }
        }

        if *key == NEW.get() {
            todo!("TODO: the new function cannot be accessed directly.")
        }

        // NOTE: If the type is an object, try to resolve its class.  It it's not an object or has
        // no class, try to find it in known types.
        let class = value
            .as_object()
            .ok()
            .and_then(|object| object.class())
            .or_else(|| self.value_class_mapping.get(&value.kind()).cloned());

        let value = self.get_method(class.as_ref(), key, &value);

        Ok(value)
    }

    pub(super) fn get_method(&self, class: Option<&Class>, key: &Symbol, receiver: &Value) -> Value {
        let value = match class {
            Some(class) => match class.method(&**key) {
                Some(method) => Value::FnBound(FnBound::new(receiver.clone(), method)),
                None => Value::Null,
            },
            None => Value::Null,
        };

        value
    }

    // TODO: Replace this mutually recursive call with an execution stack to prevent the thread's stack from overflowing.
    pub(crate) fn call_fn(&mut self, arg_count: usize) -> Result<(), Error> {
        let (function, receiver) = match self.stack.peek(arg_count) {
            Value::FnBound(fn_bound) => (fn_bound.function(), Some(fn_bound.receiver())),
            value => (value.clone(), None),
        };

        let value = match &function {
            Value::FnClosure(closure) => self.call_fn_script(arg_count, receiver, &closure.fn_script(), Some(closure.upvalues()))?,
            Value::FnScript(fn_script) => self.call_fn_script(arg_count, receiver, &fn_script, None)?,
            Value::Class(class) => self.call_class_constructor(arg_count, class, Value::Object(Object::new(class.clone())))?,
            Value::FnNative(fn_native) => self.call_fn_native(arg_count, receiver, fn_native)?,
            _ => todo!("Not a function"),
        };

        self.stack.push(value);

        Ok(())
    }

    pub(crate) fn call_class_constructor(&mut self, arg_count: usize, class: &Class, mut object: Value) -> Result<Value, Error> {
        let class = class.clone();

        if let Some(new) = class.method(&NEW) {
            let bound = Value::FnBound(FnBound::new(object.clone(), new));

            *self.stack.peek_mut(arg_count) = bound;
            self.call_fn(arg_count)?;

            // NOTE: Replace the returned object with the top of stack.
            // In most cases this will be the object itself, but this allows for native constructors
            // to override the result.
            object = self.stack.peek(0).clone();
        } else if arg_count > 0 {
            todo!("TODO: Constructor has too many parameters error.",);
        } else if class.base().filter(|base| base.method(&NEW).is_some()).is_some() {
            todo!("TODO: Class must have constructor if super has constructor.",);
        }

        // NOTE: Regardless of whether or not there was a constructor, clean up the stack.
        self.stack.pop();

        Ok(object)
    }

    fn call_fn_native(&mut self, arg_count: usize, receiver: Option<Value>, fn_native: &FnNative) -> Result<Value, Error> {
        let fn_native = fn_native.clone();
        // NOTE: Include the function/receiver slot as the first parameter to the native function call.
        let mut args = self.stack.pop_count(arg_count + 1);

        if let Some(receiver) = receiver {
            args[0] = receiver;
        }

        fn_native.call(self, &args)
    }

    fn call_fn_script(&mut self, arg_count: usize, receiver: Option<Value>, fn_script: &FnScript, parent_upvalues: Option<&[Upvalue]>) -> Result<Value, Error> {
        let slots = fn_script.bytecode().slot_count();
        let reserved = slots - arg_count;
        // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
        let stack_frame = self.stack.reserve_slots(reserved);
        // NOTE: Calling convention includes an extra parameter. This parameter is the function itself for bare functions
        // and the receiver for methods.
        let stack_frame = stack_frame.prepend(arg_count + 1);

        if let Some(receiver) = receiver {
            self.stack[stack_frame][0] = receiver;
        }

        let result = self.execute_bytecode(fn_script.bytecode(), stack_frame, parent_upvalues)?;

        // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
        self.stack.release_stack_frame(stack_frame);

        Ok(result)
    }
}
