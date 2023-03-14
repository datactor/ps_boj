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

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let n = sc.read::<usize>();
    let mut v: Vec<i32> = (0..n).map(|_| sc.read::<i32>()).collect();

    let m = sc.read::<usize>();
    let beads: Vec<i32> = (0..m).map(|_| sc.read::<i32>()).collect();

    let max_weight = v.iter().sum::<i32>();
    let mut dp = vec![vec![false; (2 * max_weight + 1) as usize]; n + 1];
    dp[0][max_weight as usize] = true;

    for i in 1..=n {
        let w = v[i - 1];
        for j in 0..=(2 * max_weight) {
            dp[i][j as usize] = dp[i - 1][j as usize] ||
                (j >= w && dp[i - 1][(j - w) as usize]);
            dp[i][j as usize] = dp[i][j as usize] ||
                (j <= -w + 2 * max_weight && dp[i - 1][(j + w) as usize]);
        }
    }

    for b in beads {
        let weight = b + max_weight;
        let mut res = "N ";
        if weight <= 2 * max_weight && dp[n][weight as usize] {
            res = "Y ";
        }
        write!(output, "{}", res)?;
    }

    Ok(())
}