use std::{
    io::{self, prelude::*},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
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
    io::stdin().read_to_string(&mut input).unwrap();
    let mut scanner = Scanner::new(&input);
    let (n, s) = (scanner.read::<usize>(), scanner.read::<i32>());

    let mut a = vec![0];

    for new in (0..n/2).map(|_| scanner.read::<i32>()) {
        for i in 0..a.len() {
            a.push(a[i] + new);
        }
    }
    a.sort();
    let mut prev = a[0];
    let mut count = 0;
    let mut a_count = vec![];
    for a_i in a {
        if prev == a_i {
            count += 1;
        } else {
            a_count.push((prev, count));
            prev = a_i;
            count = 1;
        }
    }
    a_count.push((prev, count));
    let mut b = vec![0];
    for new in scanner.input.into_iter().map(|s| s.parse::<i32>().unwrap()) {
        for i in 0..b.len() {
            b.push(b[i] + new);
        }
    }
    b.sort();
    let mut prev = b[0];
    let mut count = 0usize;
    let mut b_count = vec![];
    for bi in b {
        if prev == bi {
            count += 1;
        } else {
            b_count.push((prev, count));
            prev = bi;
            count = 1;
        }
    }
    b_count.push((prev, count));
    let mut li = 0;
    let mut re = b_count.len();
    let mut count = 0;
    while li < a_count.len() && re > 0 {
        let sum = a_count[li].0 + b_count[re - 1].0;
        if sum == s {
            count += a_count[li].1 * b_count[re - 1].1;
            re -= 1;
        } else if sum > s {
            re -= 1;
        } else {
            li += 1;
        }
    }
    if s == 0 {
        count -= 1;
    }
    println!("{}", count);
    Ok(())
}
