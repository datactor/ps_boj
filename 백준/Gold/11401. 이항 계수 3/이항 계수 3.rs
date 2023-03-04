use std::{
    error::Error,
    io::{self},
};

const P: u64 = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut v = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>())
        .flatten();
    let (n, k) = (v.next().unwrap(), v.next().unwrap());

    let a = fac(k + 1, n);
    let b = fac(2, n - k) % P;

    println!("{}", (a % P) * mod_pow(b, P - 2) % P);
    Ok(())
}

fn fac(start: u64, end: u64) -> u64 {
    let mut result = 1;
    for i in start..=end {
        result = (result * i) % P;
    }
    result
}

fn mod_pow(base: u64, exponent: u64) -> u64 {
    match exponent {
        0 => 1,
        e if e % 2 == 1 => (mod_pow(base, e - 1) * base) % P,
        _ => (mod_pow(base, exponent / 2).pow(2)) % P,
    }
}