use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            it: s.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse::<T>().ok().unwrap()
    }
}


// struct List {
//     arr: Vec<i32>,
//     val_and_len: Vec<(i32, usize)>,
// }
//
// impl List {
//     fn new() {
//
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    // let mut input = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap());

    let mut sc = Scanner::new(&input);
    let n = sc.read::<usize>();
    let mut v: VecDeque<i32> =(0..n).map(|_| sc.read()).collect();

    // let n = input.next().unwrap() as usize;
    // let mut v: VecDeque<i32> = input.take(n).collect();

    let mut lis_arr = vec![-1_000_000_001];
    let mut lis_total = vec![(-1_000_000_001, 0)];


    while !v.is_empty() {
        let num = v.pop_front().unwrap();

        if &num > lis_arr.last().unwrap() {
            lis_total.push((num, lis_arr.len()));
            lis_arr.push(num);
        } else {
            let idx = bisec(&mut lis_arr, num);
            lis_arr[idx] = num;
            lis_total.push((num, idx));
        }
    }

    let mut lis_len = lis_arr.len() - 1;
    let mut lis = Vec::new();

    while !lis_total.is_empty() && lis_len > 0 {
        let (num, idx) = lis_total.pop().unwrap();
        if idx == lis_len {
            lis.push(num);
            lis_len -= 1;
        }
    }

    writeln!(output, "{}", lis.len())?;
    lis.iter().rev().for_each(|s| write!(output, "{} ", s).unwrap());

    Ok(())
}

fn bisec(lis_arr: &mut Vec<i32>, num: i32) -> usize {
    let mut low = -1;
    let mut high = lis_arr.len() as i32;

    while low + 1 < high {
        let mid = (low + high) / 2;
        if num > lis_arr[mid as usize] {
            low = mid as i32
        } else {
            high = mid
        }
    } high as usize
}

//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////