use dice::{
    value::{NativeFn, Value},
    Dice, Runtime,
};
use std::{io::Write, rc::Rc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_current_dir(std::fs::canonicalize("data/scripts")?)?;

    let mut dice = Dice::default();
    dice.runtime().load_prelude("prelude.dm").expect("Error conversion.");
    dice.runtime()
        .add_global("print", Value::with_native_fn(Rc::new(print_value) as NativeFn))
        .expect("Error conversion.");

    loop {
        print!("Input: ");
        std::io::stdout().flush()?;

        let mut input = Vec::new();
        loop {
            let mut line = String::new();

            std::io::stdin().read_line(&mut line)?;
            writeln!(&mut input, "{}", line.trim_end().trim_end_matches('\\'))?;

            if !line.trim_end().ends_with('\\') {
                break;
            }
        }

        let start = std::time::Instant::now();
        let input = std::str::from_utf8(input.as_slice())?;

        match dice.run_script(input) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!("Result (time={} ms): {}", (elapsed.as_micros() as f64 / 1000.0), result,);
            }
            Err(err) => eprintln!("{}", err),
        };
    }
}

fn print_value(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, ()> {
    if let [_, arg, ..] = args {
        println!("{}", arg);
    }

    Ok(Value::Unit)
}

// fn err_panic(_: &mut dyn Runtime, args: &[Value]) -> Result<Value, ()> {
//     if let [_, Value::String(str), ..] = args {
//         Err(RuntimeError::Aborted(str.to_string()))
//     } else {
//         Err(RuntimeError::Aborted(String::from("Panic occurred.")))
//     }
// }
