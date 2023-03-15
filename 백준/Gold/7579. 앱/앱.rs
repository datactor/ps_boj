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

    let (n, m) = (sc.read::<usize>(), sc.read::<usize>());
    let mem: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();
    let costs: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    let mut dp = vec![0; 10001];
    dp[costs[0]] = mem[0];

    for i in 1..n {
        let cur_cost = costs[i];
        for j in (0..10001).rev() {
            if dp[j] == 0 && j > 0 {
                continue
            }
            let k = cur_cost + j;
            if dp[k] != 0 {
                dp[k] = usize::max(dp[j] + mem[i], dp[k]);
            } else {
                dp[k] = dp[j] + mem[i]
            }
        }
    }

    for i in 0..10001 {
        if dp[i] >= m {
            writeln!(output, "{}", i)?;
            break
        }
    }

    Ok(())
}