#[macro_use]
extern crate afl;

use dice::Dice;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(data) = std::str::from_utf8(data) {
            Dice::default().run_script(data);
        }
    });
}
