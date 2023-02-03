use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::BinaryHeap,
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
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());
    let mut graph = vec![Vec::new(); n+1];
    let mut ind = vec![0; n+1];
    let mut hq = BinaryHeap::new();

    (0..m).for_each(|_| {
        let (a, b) = (scanner.read::<usize>(), scanner.read::<i32>());
        graph[a].push(b);
        ind[b as usize] += 1;
    });

    for i in 1..n+1 {
        if ind[i] == 0 {
            hq.push(-(i as i32));
        }
    }

    let mut result = Vec::new();

    while !hq.is_empty() {
        let tmp = -hq.pop().unwrap();
        result.push(tmp);
        for i in &graph[tmp as usize] {
            ind[*i as usize] -= 1;
            if ind[*i as usize] == 0 {
                hq.push(i * -1);
            }
        }
    }

    for i in result {
        write!(output, "{} ", i)?;
    }

    Ok(())
}