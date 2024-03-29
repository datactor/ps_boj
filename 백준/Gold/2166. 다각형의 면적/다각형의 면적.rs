use std::io::{self, prelude::*};

fn main() {
    let mut input = io::stdin().lock().lines();

    let mut line = || input.next().unwrap().unwrap();
    let n = line().parse::<usize>().unwrap();

    let mut sum = 0;

    let mut xs = (0, 0);
    let mut ys = (0, 0);

    let (mut x0, mut y0) = (0, 0);

    for i in 0..n {
        let l = line();
        let (x, y) = l.split_once(' ').unwrap();
        let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        if i == 0 {
            (x0, y0) = (x, y);
            xs.1 = x;
            ys.1 = y;
        }
        xs = (xs.1, x);
        ys = (ys.1, y);

        sum += xs.0 * ys.1 - xs.1 * ys.0;
    }

    if x0 != xs.1 || y0 != ys.1 {
        sum += xs.1 * y0 - x0 * ys.1;
    }

    println!("{:.1}", (sum as f64 / 2.0).abs());
}