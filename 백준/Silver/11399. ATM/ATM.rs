use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v: Vec<usize> = input.split_ascii_whitespace().skip(1)
        .map(|s| s.parse::<usize>().unwrap()).collect();

    v.sort();

    let mut n = v.len();
    let mut sum = 0;

    for i in v {
        sum += i*n;
        n -= 1;
    }

    println!("{}", sum);

    Ok(())
}