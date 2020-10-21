use crate::module::ModuleLoader;
use crate::runtime::Runtime;
use dice_core::protocol::class::NEW;
use dice_core::upvalue::{Upvalue, UpvalueState};
use dice_core::value::{Class, FnBound, FnNative, FnScript, Object, Symbol, Value};
use dice_error::runtime_error::RuntimeError;

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

    pub(super) fn call_bin_op(&mut self, operator: &str, rhs: Value) -> Result<(), RuntimeError> {
        // TODO: Resolve operators from class members.
        let value = self
            .globals
            .get(&operator.into())
            .ok_or_else(|| RuntimeError::Aborted("No global operator defined.".to_owned()))?;
        let lhs = self.stack.pop();

        self.stack.push(value.clone());
        self.stack.push(lhs);
        self.stack.push(rhs);
        self.call_fn(2)?;

        Ok(())
    }

    pub(super) fn get_field(&self, key: &Symbol, value: Value) -> Result<Value, RuntimeError> {
        if value.is_object() {
            let object = value.as_object()?;
            let fields = object.fields();
            if let Some(field) = fields.get(&key) {
                return Ok(field.clone());
            }
        }

        if &**key == NEW {
            return Err(RuntimeError::Aborted(String::from(
                "TODO: the new function cannot be accessed directly.",
            )));
        }

        // TODO: Resolve the class type of built-in types.
        let value = match value.as_object()?.class() {
            Some(class) => match class.methods().get(&key) {
                Some(method) => Value::FnBound(FnBound::new(value.clone(), method.clone())),
                None => Value::Null,
            },
            None => Value::Null,
        };

        Ok(value)
    }

    // TODO: Replace this mutually recursive call with an execution stack to prevent the thread's stack from overflowing.
    pub(super) fn call_fn(&mut self, arg_count: usize) -> Result<(), RuntimeError> {
        let (function, receiver) = match self.stack.peek(arg_count) {
            Value::FnBound(fn_bound) => (fn_bound.function.clone(), Some(fn_bound.receiver.clone())),
            value => (value.clone(), None),
        };

        let value = match &function {
            Value::FnClosure(closure) => {
                self.call_fn_script(arg_count, receiver, &closure.fn_script, Some(closure.upvalues.as_ref()))?
            }
            Value::FnScript(fn_script) => self.call_fn_script(arg_count, receiver, &fn_script, None)?,
            Value::Class(class) => self.call_class_constructor(arg_count, class)?,
            Value::FnNative(fn_native) => self.call_fn_native(arg_count, receiver, fn_native)?,
            _ => return Err(RuntimeError::NotAFunction),
        };

        self.stack.push(value);

        Ok(())
    }

    fn call_class_constructor(&mut self, arg_count: usize, class: &Class) -> Result<Value, RuntimeError> {
        let class = class.clone();
        let mut object = Value::Object(Object::new(class.instance_type_id(), Value::Class(class.clone())));

        if let Some(new) = class.methods().get(&NEW.into()) {
            let bound = Value::FnBound(FnBound::new(object.clone(), new.clone()));
            *self.stack.peek_mut(arg_count) = bound;
            self.call_fn(arg_count)?;

            // NOTE: Replace the returned object with the top of stack.
            // In most cases this will be the object itself, but this allows for native constructors
            // to override the result.
            object = self.stack.peek(0).clone();
        } else if arg_count > 0 {
            return Err(RuntimeError::Aborted(String::from(
                "TODO: Constructor has too many parameters error.",
            )));
        }

        // NOTE: Regardless of whether or not there was a constructor, clean up the stack.
        self.stack.release_slots(1);

        Ok(object)
    }

    fn call_fn_native(
        &mut self,
        arg_count: usize,
        receiver: Option<Value>,
        fn_native: &FnNative,
    ) -> Result<Value, RuntimeError> {
        let fn_native = fn_native.clone();
        // NOTE: Include the function/receiver slot as the first parameter to the native function call.
        let mut args = self.stack.pop_count(arg_count + 1);

        if let Some(receiver) = receiver {
            args[0] = receiver;
        }

        fn_native.call(self, &mut args)
    }

    fn call_fn_script(
        &mut self,
        arg_count: usize,
        receiver: Option<Value>,
        fn_script: &FnScript,
        parent_upvalues: Option<&[Upvalue]>,
    ) -> Result<Value, RuntimeError> {
        if arg_count != fn_script.arity {
            return Err(RuntimeError::InvalidFunctionArgs(fn_script.arity, arg_count));
        }

        let slots = fn_script.bytecode.slot_count();
        let reserved = slots - arg_count;
        // NOTE: Reserve only the slots needed to cover locals beyond the arguments already on the stack.
        let stack_frame = self.stack.reserve_slots(reserved);
        // NOTE: Calling convention includes an extra parameter. This parameter is the function itself for bare functions
        // and the receiver for methods.
        let stack_frame = stack_frame.prepend(arg_count + 1);

        if let Some(receiver) = receiver {
            self.stack[stack_frame][0] = receiver;
        }

        let result = self.execute_bytecode(&fn_script.bytecode, stack_frame, parent_upvalues)?;

        // NOTE: Release the number of reserved slots plus the number of arguments plus a slot for the function itself.
        self.stack.release_slots(reserved + arg_count + 1);

        Ok(result)
    }
}
