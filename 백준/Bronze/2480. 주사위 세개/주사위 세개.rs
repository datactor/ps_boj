use std::io::{stdin, Read};
use std::cmp;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut v = input.split_ascii_whitespace().map(
        |s| s.parse::<i32>()).flatten();
    let a = v.next().unwrap();
    let b = v.next().unwrap();
    let c = v.next().unwrap();

    let mut result = 0;
    if a == b {
        if a == c {
            result = 10000 + a * 1000
        }
        else {
            result = 1000 + a * 100
        }
    } else if a == c {
        result = 1000 + a * 100
    } else if b == c {
        result = 1000 + b * 100
    } else {
        let ve = vec![a, b, c];
        result = ve.iter().max().unwrap() * 100
    }
    println!("{result}");
}