use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

const MOD: usize = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, l, r) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());

    let mut dp = vec![vec![0; r + 1]; l + 1];
    dp[1][1] = 1;

    let mut prev_dp = dp.clone();

    for i in 2..=n {
        for j in 1..=l {
            for k in 1..=r {
                dp[j][k] =
                    (
                        (prev_dp[j][k] * (i-2)) % MOD
                            + prev_dp[j][k-1]
                            + prev_dp[j-1][k]
                ) % MOD;
            }
        }
        prev_dp = dp.clone();
    }

    writeln!(output, "{}", dp[l][r])?;

    Ok(())
}