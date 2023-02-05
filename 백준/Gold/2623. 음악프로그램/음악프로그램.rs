use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
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
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());
    let mut v = vec![vec![]; n+1];
    let mut indegree = vec![0; n+1];


    for _ in 0..m {
        let num = scanner.read::<usize>();
        let sub: Vec<usize> = (0..num).map(|_| scanner.read::<usize>()).collect();
        for j in 0..num-1 {
            v[sub[j]].push(sub[j+1]);
            indegree[sub[j+1]] += 1;
        }
    }

    let mut dq = VecDeque::new();
    let mut result = vec![];
    for i in 1..n+1 {
        if indegree[i] == 0 {
            dq.push_back(i)
        }
    }

    while !dq.is_empty() {
        let num = dq.pop_front().unwrap();
        result.push(num);
        for i in &v[num] {
            indegree[*i] -= 1;
            if indegree[*i] == 0 {
                dq.push_back(*i)
            }
        }
    }

    if result.len() != n {
        writeln!(output, "0")?;
    } else {
        for i in result {
            writeln!(output, "{}", i)?;
        }
    }

    Ok(())
}