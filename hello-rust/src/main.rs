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
    if x == 234 {
        y = "banana";
    } else {
        y = "nonana"
    };
    
    println!("what?: {y}");

    let mut z = 44;
    println!("mutable: {z}");
    z = 234;
    println!("mutable 2: {z}");

    let rtup = (10, "elmao", 3.5);
    println!(
        "can you print tups?: {} {} {}: nop you can't directly",
        rtup.0,
        rtup.1,
        rtup.2, // yay you can have these ending commas like go!
    );

    let mut arr = [12, 234, 23, 2, 31];
    print_array(&mut arr)
}

fn print_array(arr: &mut [i32]) {
    for el in arr {
        println!("{el}")
    }
}
