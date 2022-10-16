use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    println!("wowie");
    let stdout = stdout();
    let message = String::from("hi!! hello!");
    let width = message.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let x = 234;
    println!("printing value: {x}");

    let y;
    if x == 235 {
        y = "banana";
    } else {
        y = "nonana";
    };
    
    println!("what?: {y}");

    let mut z = 44;
    println!("mutable: {z}");
    z = 234;
    println!("mutable 2: {z}");
}
