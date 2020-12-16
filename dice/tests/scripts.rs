use dice::{error::Error, value::Value, Dice};

#[test]
fn test_lazy_and_both_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("true && true")?;

    assert!(matches! {
        result,
        Value::Bool(true)
    });

    Ok(())
}

#[test]
fn test_lazy_and_lhs_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("true && false")?;

    assert!(matches! {
        result,
        Value::Bool(false)
    });

    Ok(())
}

#[test]
fn test_lazy_and_rhs_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("false && true")?;

    assert!(matches! {
        result,
        Value::Bool(false)
    });

    Ok(())
}

#[test]
fn test_lazy_and_both_false() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("false && false")?;

    assert!(matches! {
        result,
        Value::Bool(false)
    });

    Ok(())
}

// #[test]
// fn test_lazy_and_lhs_none_fails() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("none && false");
//
//     assert!(result.is_err());
//
//     Ok(())
// }
//
// #[test]
// fn test_lazy_and_rhs_none_fails() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("true && none");
//
//     assert!(result.is_err());
//
//     Ok(())
// }

#[test]
fn test_lazy_or_both_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("true || true")?;

    assert!(matches! {
        result,
        Value::Bool(true)
    });

    Ok(())
}

#[test]
fn test_lazy_or_lhs_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("true || false")?;

    assert!(matches! {
        result,
        Value::Bool(true)
    });

    Ok(())
}

#[test]
fn test_lazy_or_rhs_true() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("false || true")?;

    assert!(matches! {
        result,
        Value::Bool(true)
    });

    Ok(())
}

#[test]
fn test_lazy_or_both_false() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("false || false")?;

    assert!(matches! {
        result,
        Value::Bool(false)
    });

    Ok(())
}

// #[test]
// #[should_panic]
// fn test_lazy_or_lhs_none_fails() {
//     // -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("null || false");
//
//     assert!(result.is_err());
//
//     // Ok(())
// }
//
// #[test]
// fn test_lazy_or_rhs_none_fails() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("false || null");
//
//     assert!(result.is_err());
//
//     Ok(())
// }

#[test]
fn test_multiplication() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5 * 5 * 5")?;

    assert!(matches! {
        result,
        Value::Int(125)
    });

    Ok(())
}

#[test]
fn test_multiplication_parens_lhs() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("(5 * 5) * 5")?;

    assert!(matches! {
        result,
        Value::Int(125)
    });

    Ok(())
}

#[test]
fn test_multiplication_parens_rhs() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5 * (5 * 5)")?;

    assert!(matches! {
        result,
        Value::Int(125)
    });

    Ok(())
}

#[test]
fn test_addition() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5 + 5 + 5")?;

    assert!(matches! {
        result,
        Value::Int(15)
    });

    Ok(())
}

#[test]
fn test_precedence() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5 + 5 * 5")?;

    assert!(matches! {
        result,
        Value::Int(30)
    });

    Ok(())
}

#[test]
fn test_subtraction() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5-2")?;

    assert_eq!(result, Value::Int(3));

    Ok(())
}

#[test]
fn test_add_negative() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("5+-2")?;

    assert_eq!(result, Value::Int(3));

    Ok(())
}

#[test]
fn test_negate() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("- -5")?;

    assert_eq!(result, Value::Int(5));

    Ok(())
}

#[test]
fn test_not() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("!true")?;

    assert_eq!(result, Value::Bool(false));

    Ok(())
}

#[test]
fn test_equality() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script("2 + 3 == 5")?;

    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// #[test]
// fn test_equality_with_none() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//
//     let result = runtime.run_script("10 == none")?;
//     assert_eq!(false, *result.value::<bool>().unwrap());
//
//     let result = runtime.run_script("none == 10")?;
//     assert_eq!(false, *result.value::<bool>().unwrap());
//
//     let result = runtime.run_script("10 != none")?;
//     assert_eq!(true, *result.value::<bool>().unwrap());
//
//     let result = runtime.run_script("none != 10")?;
//     assert_eq!(true, *result.value::<bool>().unwrap());
//
//     let result = runtime.run_script("none == none")?;
//     assert_eq!(true, *result.value::<bool>().unwrap());
//
//     let result = runtime.run_script("none != none")?;
//     assert_eq!(false, *result.value::<bool>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_none() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("none")?;
//
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_object() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"object { test: 5 + 5 }"#)?;
//     let inner = result.get(&ValueKey::Symbol(Symbol::new_static("test")))?;
//
//     assert_eq!(10, *inner.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_field_access() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"object { test: 5 + 5 }.test"#)?;
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_safe_field_access() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"none?.test"#)?;
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_nested_safe_field_access() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"object { test: none }.test?.xy"#)?;
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_coalesce() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"none ?? 10"#)?;
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_complex_coalesce() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"object { test: none }.test?.xy ?? 10"#)?;
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_index_access() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"object { test: 5 + 5 }["test"]"#)?;
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_variable() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     context.add_variable(Symbol::new("test"), Value::new(5))?;
//     let result = runtime.run_script(r#"test + 5"#)?;
//
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_from_parent_scope() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     context.add_variable(Symbol::new("test"), Value::new(5))?;
//     let result = context.scoped().eval_expression(r#"test + 5"#)?;
//
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_conditional() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"if 5 == 5 { 10 } else { 12 }"#)?;
//
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_conditional_alternate() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"if 5 == 6 { 10 } else { 12 }"#)?;
//
//     assert_eq!(12, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_conditional_multiple_alternate() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"if 5 == 6 { 10 } else if 5 == 5 { 42 } else { 12 }"#)?;
//
//     assert_eq!(42, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_conditional_no_alternate() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"if 5 == 6 { 10 }"#)?;
//
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_conditional_gte_no_alternate() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"if 5 >= 6 { 10 }"#)?;
//
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_discard_expression_seps() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("5 + 5; none")?;
//
//     assert_eq!(value::None, *result.value::<value::None>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_discard_expression_seps_complex() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##"5["#op_add"](5); 15; 20; 25; 25["#op_add"](5)"##)?;
//
//     assert_eq!(30, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_discard_expression_seps_complex_if() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##"if false { 5 } if true { 10 }"##)?;
//
//     assert_eq!(10, *result.value::<i64>().unwrap());
//
//     Ok(())
// }
//
// #[test]
// fn test_method_call() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("5.to_Error()")?;
//     let actual = result.value::<DiceError>().unwrap();
//
//     assert_eq!("5", &**actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_method_call_with_index() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##"5["#op_add"](5)"##)?;
//     let actual = result.value::<i64>().unwrap();
//
//     assert_eq!(10, *actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_method_call_with_invalid_index() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##"5[5.0]"##);
//
//     assert!(matches!(result, Err(Error::InvalidKeyType(_))));
//
//     Ok(())
// }
//
// #[test]
// fn test_chained_method_call() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##"5["#op_add"](5).to_Error()"##)?;
//     let actual = result.value::<DiceError>().unwrap();
//
//     assert_eq!("10", &**actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_int_constructor() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("Int(5)")?;
//     let actual = result.value::<i64>().unwrap();
//
//     assert_eq!(5, *actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_int_constructor_with_float() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script("Int(5.99)")?;
//     let actual = result.value::<i64>().unwrap();
//
//     assert_eq!(5, *actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_int_constructor_with_Error() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"Int("5")"#)?;
//     let actual = result.value::<i64>().unwrap();
//
//     assert_eq!(5, *actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_Error_concat() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r##""test" + "value""##)?;
//     let actual = result.value::<DiceError>().unwrap();
//
//     assert_eq!("testvalue", &**actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_Error_concat_with_number() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#""test" + 5"#)?;
//     let actual = result.value::<DiceError>().unwrap();
//
//     assert_eq!("test5", &**actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_list_concat() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"[5] + [5, 5]"#)?;
//     let actual = result.value::<List>().unwrap().as_ref();
//
//     assert_eq!(3, actual.len());
//
//     Ok(())
// }
//
// #[test]
// fn test_list_concat_with_value() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"[5] + 5"#)?;
//     let actual = result.value::<List>().unwrap().as_ref();
//
//     assert_eq!(2, actual.len());
//
//     Ok(())
// }
//
// #[test]
// fn test_list_index() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"[5][0]"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(5, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_list_negative_index() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"[5][-1]"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(5, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_list_negative_index_out_of_bounds() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"[5][-2]"#);
//
//     assert!(matches!(result, Err(Error::IndexOutOfBounds(1, -1))));
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_decl() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = 5;"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(5, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_decl_followed_by_expression() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = 5; x + 5"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(10, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_decl_followed_by_assignment() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = 5; x = x + 10; x"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(15, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_decl_with_block_expression() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = { let x = 20; x * 2 };"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(40, actual);
//
//     Ok(())
// }
//
// #[test]
// fn test_variable_decl_with_block_expression_nested_in_expression() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = { let x = 20; x * 2 } + 2;"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(42, actual);
//
//     Ok(())
// }
//
#[test]
fn test_while_loop() -> Result<(), Error> {
    let mut runtime = Dice::default();
    let result = runtime.run_script(r#"let mut x = 0 while x < 10 { x = x + 1 } x"#)?;

    assert!(matches! {
        result,
        Value::Int(10)
    });

    Ok(())
}
//
// #[test]
// fn test_for_loop() -> Result<(), Error> {
//     let mut runtime = Dice::default();
//     let result = runtime.run_script(r#"let x = 0; for y in 0..10 { x = x + y } x"#)?;
//     let actual = *result.value::<i64>().unwrap();
//
//     assert_eq!(45, actual);
//
//     Ok(())
// }
//
