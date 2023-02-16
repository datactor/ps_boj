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

fn get_index(i: usize, j: usize, k: usize) -> usize {
    i * k + j
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let t = sc.read::<usize>();
    for _ in 0..t {
        let k = sc.read::<usize>();
        let v: Vec<usize> = (0..k).map(|_| sc.read::<usize>()).collect();
        let mut dp = vec![0; k * k];

        for i in 0..k-1 {
            dp[get_index(i, i+1, k)] = v[i] + v[i+1];
            for j in i+2..k {
                dp[get_index(i, j, k)] = dp[get_index(i, j-1, k)] + v[j];
            }
        }

        for i in 2..k {
            for j in 0..k-i {
                let mut min_sum = std::usize::MAX;
                for s in j..j+i {
                    let left_sum = dp[get_index(j, s, k)];
                    let right_sum = dp[get_index(s+1, j+i, k)];
                    min_sum = std::cmp::min(min_sum, left_sum + right_sum);
                }
                dp[get_index(j, j+i, k)] += min_sum;
            }
        }
        writeln!(output, "{}", dp[get_index(0, k-1, k)])?;
    }

    Ok(())
}
