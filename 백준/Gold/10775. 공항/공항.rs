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

fn find(x: usize, parents: &mut Vec<usize>) -> usize {
    if x == parents[x] {
        return x
    }
    parents[x] = find(parents[x], parents);
    return parents[x]
}

fn union(mut x: usize, mut y: usize, parents: &mut Vec<usize>) {
    (x, y) = (find(x, parents), find(y, parents));
    parents[y] = x;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let g = scanner.read::<usize>();
    let planes: Vec<usize> = scanner.input.skip(1).map(|s| s.parse::<usize>().unwrap()).collect();

    let mut parents: Vec<usize> = (0..g+1).map(|i| i).collect();
    let mut cnt = 0;

    for plane in planes {
        let tmp = find(plane, &mut parents);
        if tmp == 0 {
            break
        }
        union(tmp-1, tmp, &mut parents);
        cnt += 1;
    }
    println!("{cnt}");

    Ok(())
}