use super::context::Environment;
use crate::{
    runtime::{
        error::RuntimeError,
        object::{instance::ObjectInstance, key::ObjectKey},
        symbol::{
            common::{
                operators::{OP_ADD, OP_AND, OP_DIV, OP_EQ, OP_GT, OP_GTE, OP_LT, OP_LTE, OP_MUL, OP_NE, OP_NEG, OP_NOT, OP_OR, OP_REM, OP_SUB},
                types::{TY_BOOL, TY_NONE},
            },
            Symbol,
        },
        types::{anonymous::AnonymouseObject, list::List, string::RcString},
    },
    syntax::{BinaryOperator, Expression, Literal, UnaryOperator},
};
use std::{collections::HashMap, iter};

#[inline]
pub fn eval(expr: &Expression, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    eval_expression(expr, environment)
}

fn eval_expression(expr: &Expression, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    match expr {
        Expression::Literal(literal) => eval_literal(literal, environment),
        Expression::SafeAccess(expr, field) => eval_safe_field_access(expr, field, environment),
        Expression::FieldAccess(expr, field) => eval_field_access(expr, field, environment),
        Expression::FunctionCall(expr, args) => eval_function_call(expr, args, environment),
        Expression::Index(expr, index) => eval_index(expr, index, environment),
        Expression::Unary(op, expr) => eval_unary(op, expr, environment),
        Expression::Binary(op, lhs, rhs) => eval_binary(op, lhs, rhs, environment),
        Expression::Range(_op, _lower, _upper) => Err(RuntimeError::Aborted),
        Expression::Conditional(condition, body, alternate) => eval_conditional(condition, body, alternate.as_deref(), environment),
    }
}

fn eval_literal(literal: &Literal, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    match literal {
        Literal::Identifier(ref symbol) => environment.variable(symbol),
        Literal::None => Ok(ObjectInstance::NONE),
        Literal::Integer(int) => Ok(ObjectInstance::new(*int)),
        Literal::Float(float) => Ok(ObjectInstance::new(*float)),
        Literal::String(string) => Ok(ObjectInstance::new(Into::<RcString>::into(string.clone()))),
        Literal::Boolean(bool) => Ok(ObjectInstance::new(*bool)),
        Literal::List(list) => eval_list_literal(list, environment),
        Literal::Object(object) => eval_object_literal(object, environment),
    }
}

#[inline]
fn eval_list_literal(list: &[Expression], environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let result: List = list
        .iter()
        .map(|expr| eval_expression(expr, environment))
        .collect::<Result<Vec<_>, _>>()?
        .into();

    Ok(ObjectInstance::new(result))
}

#[inline]
fn eval_object_literal(object: &HashMap<ObjectKey, Expression>, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let result = object
        .iter()
        .map(|(key, value)| Ok::<_, RuntimeError>((key.clone(), eval_expression(value, environment)?)))
        .collect::<Result<_, _>>()?;

    Ok(ObjectInstance::new(AnonymouseObject::new(result)))
}

fn eval_function_call(expr: &Expression, args: &[Expression], environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    match expr {
        Expression::Literal(Literal::Identifier(target)) => {
            let args = args.iter().map(|arg| eval_expression(arg, environment)).collect::<Result<Vec<_>, _>>()?;
            environment.variable(target)?.call(&args)
        }
        Expression::FieldAccess(this, method) => {
            let method = &ObjectKey::Symbol(method.clone());
            eval_method_call(&method, this, args, environment)
        }
        Expression::Index(this, method) => {
            let method = eval_object_key(method, environment)?;
            eval_method_call(&method, this, args, environment)
        }
        _ => unreachable!(),
    }
}

#[inline]
fn eval_method_call(method: &ObjectKey, this: &Expression, args: &[Expression], environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let args = iter::once_with(|| eval_expression(this, environment))
        .chain(args.iter().map(|arg| eval_expression(arg, environment)))
        .collect::<Result<Vec<ObjectInstance>, RuntimeError>>()?;

    match args.first() {
        Some(this) => {
            let this = this.get(method)?;
            this.call(&args)
        }
        None => Err(RuntimeError::NoSelfParameterProvided),
    }
}

#[inline]
fn eval_index(expr: &Expression, index: &Expression, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let target = eval_expression(expr, environment)?;
    let index = eval_object_key(index, environment)?;

    target.get(&index)
}

fn eval_unary(op: &UnaryOperator, expr: &Expression, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    match op {
        UnaryOperator::Negate => object_ref.get(&ObjectKey::Symbol(OP_NEG))?.call(&[object_ref]),
        UnaryOperator::Not => object_ref.get(&ObjectKey::Symbol(OP_NOT))?.call(&[object_ref]),
    }
}

fn eval_binary(op: &BinaryOperator, lhs: &Expression, rhs: &Expression, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let lhs = eval_expression(lhs, environment)?;
    // TODO: Only evaluate this when needed.
    let rhs = eval_expression(rhs, environment)?;

    match op {
        BinaryOperator::DiceRoll => Err(RuntimeError::Aborted),
        BinaryOperator::Multiply => lhs.get(&ObjectKey::Symbol(OP_MUL))?.call(&[lhs, rhs]),
        BinaryOperator::Divide => lhs.get(&ObjectKey::Symbol(OP_DIV))?.call(&[lhs, rhs]),
        BinaryOperator::Remainder => lhs.get(&ObjectKey::Symbol(OP_REM))?.call(&[lhs, rhs]),
        BinaryOperator::Add => lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(&[lhs, rhs]),
        BinaryOperator::Subtract => lhs.get(&ObjectKey::Symbol(OP_SUB))?.call(&[lhs, rhs]),
        BinaryOperator::Equals => lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(&[lhs, rhs]),
        BinaryOperator::NotEquals => lhs.get(&ObjectKey::Symbol(OP_NE))?.call(&[lhs, rhs]),
        BinaryOperator::GreaterThan => lhs.get(&ObjectKey::Symbol(OP_GT))?.call(&[lhs, rhs]),
        BinaryOperator::GreaterThanOrEquals => lhs.get(&ObjectKey::Symbol(OP_GTE))?.call(&[lhs, rhs]),
        BinaryOperator::LessThan => lhs.get(&ObjectKey::Symbol(OP_LT))?.call(&[lhs, rhs]),
        BinaryOperator::LessThanOrEquals => lhs.get(&ObjectKey::Symbol(OP_LTE))?.call(&[lhs, rhs]),
        BinaryOperator::LogicalAnd => lhs.get(&ObjectKey::Symbol(OP_AND))?.call(&[lhs, rhs]),
        BinaryOperator::LogicalOr => lhs.get(&ObjectKey::Symbol(OP_OR))?.call(&[lhs, rhs]),
        BinaryOperator::Coalesce => {
            if *lhs.reflect_type().name() != TY_NONE {
                Ok(lhs)
            } else {
                Ok(rhs)
            }
        }
        BinaryOperator::Discard => Ok(rhs),
    }
}

#[inline]
fn eval_safe_field_access(expr: &Expression, field: &Symbol, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    if *object_ref.reflect_type().name() != TY_NONE {
        object_ref.get(&ObjectKey::Symbol(field.clone()))
    } else {
        Ok(ObjectInstance::NONE)
    }
}

#[inline]
fn eval_field_access(expr: &Expression, field: &Symbol, environment: &Environment) -> Result<ObjectInstance, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;
    object_ref.get(&ObjectKey::Symbol(field.clone()))
}

#[inline]
fn eval_object_key(expr: &Expression, environment: &Environment) -> Result<ObjectKey, RuntimeError> {
    let index = eval_expression(expr, environment)?;

    if let Some(index) = index.value::<i64>() {
        Ok(ObjectKey::Index(*index))
    } else if let Some(index) = index.value::<RcString>() {
        let index: String = index.to_string();
        Ok(ObjectKey::Symbol(Symbol::new(index)))
    } else {
        Err(RuntimeError::InvalidKeyType(index.reflect_type().name().clone()))
    }
}

fn eval_conditional(
    condition: &Expression,
    body: &Expression,
    alternate: Option<&Expression>,
    environment: &Environment,
) -> Result<ObjectInstance, RuntimeError> {
    let condition_result = eval_expression(condition, environment)?;
    let condition = *condition_result
        .value::<bool>()
        .ok_or_else(|| RuntimeError::InvalidType(TY_BOOL, condition_result.reflect_type().name().clone()))?;

    if condition {
        eval_expression(body, environment)
    } else if let Some(alternate) = alternate {
        eval_expression(alternate, environment)
    } else {
        Ok(ObjectInstance::NONE)
    }
}
