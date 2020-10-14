use dice::{Dice, Runtime, RuntimeError, Value};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Dice::default();
    runtime.register_native_fn("print", print_value);
    runtime.register_native_fn("filter", filter);

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

        match runtime.run_script(input) {
            Ok(result) => {
                let elapsed = start.elapsed();
                let type_id = result.type_id();
                println!(
                    "Result (time={} ms, typeid={:#10X}): {}",
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

fn filter(runtime: &mut dyn Runtime, args: &[Value]) -> Result<Value, RuntimeError> {
    if let [_, list, filter_fn, ..] = args {
        let list = list.as_list()?;
        let mut result = Vec::new();

        for item in &*list.elements() {
            let included = runtime.call_function(filter_fn.clone(), &[item.clone()])?.as_bool()?;

            if included {
                result.push(item.clone());
            }
        }

        return Ok(Value::List(result.into()));
    }

    todo!("Error out here.")
}
