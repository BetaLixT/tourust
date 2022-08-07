use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("hi!! hello!");
    let width = message.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
