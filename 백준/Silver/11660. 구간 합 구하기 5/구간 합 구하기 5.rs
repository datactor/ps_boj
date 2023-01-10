use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();
    let n = first_line.next().unwrap();
    let m = first_line.next().unwrap();

    let v: Vec<Vec<i32>> = (0..n).map(|_| lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<i32>().unwrap()).collect()
    ).collect();
    let mut arr = vec![vec![0; n+1]; n+1];

    for i in 1..n+1 {
        for j in 1..n+1 {
            arr[i][j] = arr[i][j-1] + arr[i-1][j] - arr[i-1][j-1] + v[i-1][j-1];
        }
    }

    for _ in 0..m {
        let mut pos = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>()).flatten();
        let (x1, y1, x2, y2) =
            (pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap());
        writeln!(output, "{}", arr[x2][y2] - arr[x1-1][y2] - arr[x2][y1-1] + arr[x1-1][y1-1])?
    }
    Ok(())
}
