use std::collections::VecDeque;
use std::io::{self, prelude::*, BufWriter};

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

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let n = sc.read::<usize>();

    let table: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    let mut ans = 0;

    let mut dq = VecDeque::new();
    for i in 0..n {
        while !dq.is_empty() && table[*dq.back().unwrap()] > table[i] {
            let tmp = dq.pop_back().unwrap();

            let width = if dq.is_empty() {
                i
            } else {
                i - dq.back().unwrap() - 1
            };
            ans = usize::max(ans, width * table[tmp]);
        }
        dq.push_back(i);
    }
    while !dq.is_empty() {
        let tmp = dq.pop_back().unwrap();

        let width = if dq.is_empty() {
            n
        } else {
            n - dq.back().unwrap() - 1
        };
        ans = usize::max(ans, width * table[tmp])
    }

    writeln!(output, "{}", ans)?;

    Ok(())
}