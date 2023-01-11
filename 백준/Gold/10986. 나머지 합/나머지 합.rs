use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = first_line.next().unwrap();
    let m = first_line.next().unwrap();

    let mut arr: Vec<i64> = vec![0; m];
    arr[0] = 1;

    let v: Vec<usize> = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>().unwrap()).collect();

    let mut total = 0;
    for i in 0..n {
        total += v[i];
        let r = total % m;
        arr[r] += 1
    }

    println!("{}", (0..m).map(|i| arr[i] * (arr[i] - 1) / 2).sum::<i64>());

    Ok(())
}