use std::{
    io::{self, prelude::*},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, c) = (scanner.read::<usize>(), scanner.read::<i32>());

    let mut nodes: Vec<i32> = (0..n).map(|_| scanner.read::<i32>()).collect();
    nodes.sort();

    let max = (nodes[n-1] - nodes[0]) / (c-1);
    let (mut left, mut right) = (1, max);

    while left <= right {
        let mid = (left + right) / 2;
        let mut cnt = 1;
        let mut dist = nodes[0];
        for i in nodes[1..n].iter() {
            if i - mid >= dist {
                dist = *i;
                cnt += 1
            }
        }
        if cnt >= c {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    println!("{}", right);

    Ok(())
}