use std::{
    io::{self, prelude::*},
    error::Error,
};

static mut W: usize = 0;
static mut B: usize = 0;

fn main () -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut n = lines.next().unwrap().parse::<usize>().unwrap();

    let v: Vec<Vec<usize>> = (0..n).map(|_| lines.next().unwrap().split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect()).collect();

    unsafe {
        div_conq(0, 0, n, &v);
        println!("{}\n{}", W, B);
    }
    Ok(())
}

unsafe fn div_conq(x: usize, y: usize, n: usize, v: &Vec<Vec<usize>>) {
    let mut tmp = 0;
    for i in x..x+n {
        for j in y..y+n {
            if v[i][j] == 1 {
                tmp += 1
            }
        }
    }
    if tmp == 0 {
        W += 1;
    } else if tmp == n.pow(2) {
        B += 1;
    } else {
        div_conq(x, y, n / 2, v);
        div_conq(x + n / 2, y, n / 2, v);
        div_conq(x, y + n / 2, n / 2, v);
        div_conq(x + n/2, y + n / 2, n / 2, v);
    }
    return
}