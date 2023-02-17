use std::{
    io::{self, prelude::*},
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
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let n = sc.read::<usize>();
    let mut arr: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    let mut swap = 0;

    merge_sort(0, n, &mut arr, &mut swap);

    println!("{}", swap);
    Ok(())
}

fn merge_sort(start: usize, end: usize, arr: &mut Vec<usize>, swap: &mut i64) {
    let size = end - start;
    let mid = (start + end) / 2;
    if size <= 1 {
        return;
    }

    merge_sort(start, mid, arr, swap);
    merge_sort(mid, end, arr, swap);

    let mut new_arr = Vec::with_capacity(size);
    let (mut idx1, mut idx2) = (start, mid);
    let mut cnt = 0;

    while idx1 < mid && idx2 < end {
        if arr[idx1] > arr[idx2] {
            new_arr.push(arr[idx2]);
            idx2 += 1;
            cnt += 1;
        } else {
            new_arr.push(arr[idx1]);
            idx1 += 1;
            *swap += cnt;
        }
    }

    while idx1 < mid {
        new_arr.push(arr[idx1]);
        idx1 += 1;
        *swap += cnt;
    }

    while idx2 < end {
        new_arr.push(arr[idx2]);
        idx2 += 1;
    }

    arr[start..end].copy_from_slice(&new_arr[..]);
}