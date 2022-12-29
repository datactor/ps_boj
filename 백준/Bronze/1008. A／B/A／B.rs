use std::io::{self, prelude::*};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let a = v.next().unwrap();
    let b = v.next().unwrap();

    println!("{:.10}", (a as f64)/(b as f64));
}