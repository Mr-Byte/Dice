use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dice::{Dice, Runtime};
use std::time::Duration;

fn loop_in_place_addition(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("in-place-addition", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 100000 { x += 1; }"))
                .unwrap()
        })
    });
}

fn loop_addition_with_assignment(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("addition-with-assignment", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 100000 { x = x + 1; }"))
                .unwrap()
        })
    });
}

fn range_for_loop_addition_with_assignment(criterion: &mut Criterion) {
    let mut dice = Dice::default();
    dice.runtime().load_prelude("data/scripts/prelude.dm").unwrap();

    criterion.bench_function("range-for-loop-addition-with-assignment", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; for n in 0..100000 { x += 1; }"))
                .unwrap()
        })
    });
}

fn iterator_for_loop_addition_with_assignment(criterion: &mut Criterion) {
    let mut dice = Dice::default();
    dice.runtime().load_prelude("data/scripts/prelude.dm").unwrap();

    criterion.bench_function("iterator-for-loop-addition-with-assignment", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box(
                "let mut x = 0; let xs = 0..100000; for n in xs.iter() { x += 1; }",
            ))
            .unwrap()
        })
    });
}

fn loop_function_call(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("function-call", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box(
                "fn one() { 1 } let mut n = 0; while n < 100000 { n += one(); }",
            ))
            .unwrap()
        })
    });
}

fn loop_closure_call(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("loop-closure-call", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; let f = || x += 1; while x < 100000 { f(); }"))
                .unwrap()
        })
    });
}

fn closure_called_by_another_function_in_parent_scope(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("closure-called-by-closure-in-same-parent", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box(
                "fn test() { let x = 42; fn foo() { x } fn bar(f) { f() } bar(foo) } test()",
            ))
            .unwrap()
        })
    });
}

fn closure_called_outside_declaring_scope(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("closure-called-outside-declaring-scope", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box(
                "fn test() { let mut x = 0; fn inner() { x = x + 1; x } } let s = test(); s();",
            ))
            .unwrap()
        })
    });
}

criterion_group!(
    name = loops;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(10)).nresamples(5000);
    targets = loop_in_place_addition,
        loop_addition_with_assignment,
        loop_function_call,
        loop_closure_call,
        range_for_loop_addition_with_assignment,
        iterator_for_loop_addition_with_assignment
);

criterion_group!(
    name = closures;
    config = Criterion::default().sample_size(500).measurement_time(Duration::from_secs(10)).nresamples(5000);
    targets = closure_called_by_another_function_in_parent_scope, closure_called_outside_declaring_scope
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_current_dir(
        std::env::current_dir()?
            .parent()
            .expect("Parent directory should exist."),
    )?;

    loops();
    closures();

    Criterion::default().configure_from_args().final_summary();

    Ok(())
}
