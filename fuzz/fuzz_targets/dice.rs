#![no_main]
use dice::Dice;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: String| {
    Dice::default().run_script(&data);
});
