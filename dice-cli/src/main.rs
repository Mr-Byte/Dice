use dice::value::Value;
use dice::{Dice, Runtime, RuntimeError};
use std::io::Write;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_current_dir(std::fs::canonicalize("data/scripts")?)?;

    let mut dice = Dice::default();
    dice.runtime().register_native_function("print", Rc::new(print_value));
    dice.runtime().load_prelude("prelude.dm")?;

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

        let input = String::from_utf8(input)?;
        let start = std::time::Instant::now();

        match dice.run_script(input) {
            Ok(result) => {
                let elapsed = start.elapsed();
                let type_id = result.type_id();
                println!(
                    "Result (time={} ms, typeid={}): {}",
                    (elapsed.as_micros() as f64 / 1000.0),
                    type_id,
                    result,
                );
            }
            Err(err) => eprintln!("{}", err),
        };
    }
}

fn print_value(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, arg, ..] = args {
        println!("{}", arg);
    }

    Ok(Value::Unit)
}
