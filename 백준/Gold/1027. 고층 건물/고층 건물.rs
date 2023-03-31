use std::{
    io::{self, prelude::*},
};

fn main() {
    let mut input = io::stdin().lock().lines();
    let mut read = || input.next().unwrap().unwrap();
    let n = read().parse::<usize>().unwrap();
    let buildings = read().split_ascii_whitespace().take(n).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut cnt: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut max_gradient = f64::MIN;
        for j in (i + 1)..n {
            let h = buildings[j] - buildings[i];
            let w = j - i;
            let g = h as f64 / w as f64;

            if g <= max_gradient {
                continue;
            }
            max_gradient = g;
            cnt[i] += 1;
            cnt[j] += 1;
        }
    }

    println!("{}", cnt.iter().max().unwrap());
}