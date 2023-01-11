use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();
    let m = v.next().unwrap();

    let mut res: Vec<i64> = vec![0; m];

    let mut total = 0;
    for _ in 0..n {
        total += v.next().unwrap();
        res[total % m] += 1;
    }

    println!("{}", res[0] + (0..m).map(|i| res[i] * (res[i] - 1) / 2).sum::<i64>());
    Ok(())
}