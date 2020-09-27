use dice::{Dice, NativeError, Runtime, RuntimeError, Value};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Dice::default();
    runtime.register_native_fn("print", print_value);

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

        match runtime.run_script(&input) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!("Result ({} ms): {:?}", (elapsed.as_micros() as f64 / 1000.0), result);
            }
            Err(err) => eprintln!("{}", err),
        };
    }
}

fn print_value(_runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, NativeError> {
    if let [arg, ..] = args {
        println!("{}", arg);
    }

    return Ok(Value::Unit);
}
