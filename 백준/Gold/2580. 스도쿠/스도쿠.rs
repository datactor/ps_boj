use std::{
    io::{self, prelude::*, BufWriter, StdoutLock},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut blank = Vec::new();
    let mut v = (0..9)
        .map(|i| {
            let line: Vec<usize> = lines.next().unwrap()
                .split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            (0..9).for_each(|j| if line[j] == 0 { blank.push((i, j)) });
            line
        })
        .collect();

    dfs(0, &blank, &mut v, &mut output);

    Ok(())
}

fn check_row(x: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        if a == v[x][i] {
            return false
        }
    } true
}

fn check_col(y: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        if a == v[i][y] {
            return false
        }
    } true
}

fn check_sqr(x: usize, y: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    let (nx, ny) = (x / 3 * 3, y / 3 * 3);
    for i in 0..3 {
        for j in 0..3 {
            if a == v[nx+i][ny+j] {
                return false
            }
        }
    } true
}

fn dfs(idx: usize, blank: &Vec<(usize, usize)>, v: &mut Vec<Vec<usize>>, output: &mut BufWriter<StdoutLock>) {
    if idx == blank.len() {
        for i in 0..9 {
            for j in 0..9 {
                write!(output, "{} ", v[i][j]).unwrap();
            }
            write!(output, "\n").unwrap();
        }
        output.flush().unwrap();
        std::process::exit(0);
    }

    let (x, y) = (blank[idx].0, blank[idx].1);
    
    for i in 1..10 {
        if check_row(x, i, v) && check_col(y, i, v) && check_sqr(x, y, i, v) {
            v[x][y] = i;
            dfs(idx + 1, blank, v, output);
            v[x][y] = 0;
        }
    }
}