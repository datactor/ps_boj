use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

const P: u64 = 1_000_000_007;

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, r) = (sc.read::<u64>(), sc.read::<u64>());

    let a = fac(r + 1, n);
    let b = fac(2, n - r) % P;

    writeln!(output, "{}", (a % P) * sqr(b, P - 2) % P)?;

    Ok(())
}

fn fac(s: u64, n: u64) -> u64 {
    let mut tmp = 1;
    for i in s..=n {
        tmp = (tmp * i) % P;
    }
    tmp
}

fn sqr(a: u64, b: u64) -> u64 {
    match b {
        0 => 1,
        b if b % 2 == 1 => (sqr(a, b - 1) * a) % P,
        _ => (sqr(a, b / 2).pow(2)) % P,
    }
}