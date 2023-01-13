use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = first_line.next().unwrap();
    let m = first_line.next().unwrap();
    let k = first_line.next().unwrap();

    let board: Vec<_> = (0..n).map(|_|
        lines.next().unwrap().as_bytes()
    ).collect();

    let mut arr_b = vec![vec![0; m+1]; n+1];
    let mut arr_w = vec![vec![0; m+1]; n+1];

    let mut min_b = vec![vec![0; m+1]; n+1];
    let mut min_w = vec![vec![0; m+1]; n+1];

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if (i+j-2)%2 == 0 {
                if board[i-1][j-1] == 87 {
                    arr_b[i-1][j-1] += 1
                } else {
                    arr_w[i-1][j-1] += 1
                }
            } else {
                if board[i-1][j-1] == 87 {
                    arr_w[i-1][j-1] += 1
                } else {
                    arr_b[i-1][j-1] += 1
                }
            }
            if j >= k {
                if i-1 == 0 {
                    min_b[i-1][j-1] = arr_b[i-1][j-k..j].iter().sum::<i32>();
                    min_w[i-1][j-1] = arr_w[i-1][j-k..j].iter().sum::<i32>();
                } else {
                    min_b[i-1][j-1] = min_b[i-2][j-1] + arr_b[i-1][j-k..j].iter().sum::<i32>();
                    min_w[i-1][j-1] = min_w[i-2][j-1] + arr_w[i-1][j-k..j].iter().sum::<i32>();
                }
            }
        }
    }

    let mut min = 2000_000;
    for i in k-1..n {
        for j in k-1..m {
            if i < k {
                min = min.min(min_w[i][j]);
                min = min.min(min_b[i][j]);
            } else {
                min = min.min(min_w[i][j] - min_w[i-k][j]);
                min = min.min(min_b[i][j] - min_b[i-k][j]);
            }
        }
    }

    println!("{}", min);

    Ok(())
}