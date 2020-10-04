macro_rules! op {
    (OP_EQ, $lhs:expr, $rhs:expr) => {
        $lhs == $rhs
    };
    (OP_NEQ, $lhs:expr, $rhs:expr) => {
        $lhs != $rhs
    };
    (OP_GT, $lhs:expr, $rhs:expr) => {
        $lhs > $rhs
    };
    (OP_GTE, $lhs:expr, $rhs:expr) => {
        $lhs >= $rhs
    };
    (OP_LT, $lhs:expr, $rhs:expr) => {
        $lhs < $rhs
    };
    (OP_LTE, $lhs:expr, $rhs:expr) => {
        $lhs <= $rhs
    };
}

// TODO: Change EQ and NEQ to use the peek(0) method to in-place mutate the top of the stack.
#[macro_export]
macro_rules! comparison_op {
    ($stack:expr, OP_EQ) => {
        match ($stack.pop(), $stack.pop()) {
            (dice_core::value::Value::Null, dice_core::value::Value::Null) => {
                $stack.push(dice_core::value::Value::Bool(true))
            }
            (dice_core::value::Value::Null, _) => $stack.push(dice_core::value::Value::Bool(false)),
            (_, dice_core::value::Value::Null) => $stack.push(dice_core::value::Value::Bool(false)),
            (dice_core::value::Value::Unit, dice_core::value::Value::Unit) => {
                $stack.push(dice_core::value::Value::Bool(true))
            }
            (dice_core::value::Value::Unit, _) => $stack.push(dice_core::value::Value::Bool(false)),
            (_, dice_core::value::Value::Unit) => $stack.push(dice_core::value::Value::Bool(false)),
            (dice_core::value::Value::Bool(lhs), dice_core::value::Value::Bool(rhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_EQ, lhs, rhs)))
            }
            (dice_core::value::Value::Int(lhs), dice_core::value::Value::Int(rhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_EQ, lhs, rhs)))
            }
            (dice_core::value::Value::Float(lhs), dice_core::value::Value::Float(rhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_EQ, lhs, rhs)))
            }
            (dice_core::value::Value::FnClosure(lhs), dice_core::value::Value::FnClosure(rhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_EQ, lhs, rhs)))
            }
            _ => todo!(),
        }
    };

    ($stack:expr, OP_NEQ) => {
        match ($stack.pop(), $stack.pop()) {
            (dice_core::value::Value::Null, dice_core::value::Value::Null) => {
                $stack.push(dice_core::value::Value::Bool(false))
            }
            (dice_core::value::Value::Null, _) => $stack.push(dice_core::value::Value::Bool(true)),
            (_, dice_core::value::Value::Null) => $stack.push(dice_core::value::Value::Bool(true)),
            (dice_core::value::Value::Unit, dice_core::value::Value::Unit) => {
                $stack.push(dice_core::value::Value::Bool(false))
            }
            (dice_core::value::Value::Unit, _) => $stack.push(dice_core::value::Value::Bool(true)),
            (_, dice_core::value::Value::Unit) => $stack.push(dice_core::value::Value::Bool(true)),
            (dice_core::value::Value::Bool(rhs), dice_core::value::Value::Bool(lhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_NEQ, lhs, rhs)))
            }
            (dice_core::value::Value::Int(rhs), dice_core::value::Value::Int(lhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_NEQ, lhs, rhs)))
            }
            (dice_core::value::Value::Float(rhs), dice_core::value::Value::Float(lhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_NEQ, lhs, rhs)))
            }
            (dice_core::value::Value::FnClosure(rhs), dice_core::value::Value::FnClosure(lhs)) => {
                $stack.push(dice_core::value::Value::Bool(op!(OP_NEQ, lhs, rhs)))
            }
            _ => todo!(),
        }
    };

    ($stack:expr, $op:ident) => {
        match ($stack.pop(), $stack.peek(0)) {
            (dice_core::value::Value::Bool(rhs), dice_core::value::Value::Bool(lhs)) => *lhs = op!($op, *lhs, rhs),
            (dice_core::value::Value::Int(rhs), dice_core::value::Value::Int(lhs)) => {
                *$stack.peek(0) = dice_core::value::Value::Bool(op!($op, *lhs, rhs))
            }
            (dice_core::value::Value::Float(rhs), dice_core::value::Value::Float(lhs)) => {
                *$stack.peek(0) = dice_core::value::Value::Bool(op!($op, *lhs, rhs))
            }
            _ => todo!(),
        }
    };
}
