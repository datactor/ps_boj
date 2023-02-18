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

    let mut s = vec![vec![vec![0; r + 1]; l + 1]; n + 1];
    s[1][1][1] = 1;

    for i in 2..=n {
        for j in 1..=l {
            for k in 1..=r {
                s[i][j][k] = ((s[i - 1][j][k] * (i - 2)) % MOD
                    + s[i - 1][j][k - 1]
                    + s[i - 1][j - 1][k])
                    % MOD;
            }
        }
    }

    writeln!(output, "{}", s[n][l][r])?;

    Ok(())
}