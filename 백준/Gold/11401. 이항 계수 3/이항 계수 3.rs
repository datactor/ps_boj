// Fermat's little Theorem

use std::{
    io::{self},
    error::Error,
};

static P: u128 = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(|s| s.parse::<u128>()).flatten();
    let (n, k) = (v.next().unwrap(), v.next().unwrap());
    
    let a = fac(n);
    let b = (fac(n-k) * fac(k)) % P;

    println!("{}", (a%P) * sqr(b, P-2) % P);
    Ok(())
}

fn fac(mut n: u128) -> u128{
    let mut tmp = 1;
    for i in 2..n+1 {
        tmp = (tmp * i) % P;
    } tmp
}

fn sqr(a: u128, b: u128) -> u128 {
    if b == 0 {
        return 1
    } else if b % 2 == 1 {
        return (sqr(a, b/2).pow(2) * a) % P
    } else {
        return (sqr(a, b/2).pow(2)) % P
    }
}