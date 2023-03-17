use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let input = input.split_ascii_whitespace();
    let a: Vec<i32> = input.skip(1).into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let n = a.len();

    let mut stack = Vec::with_capacity(n);
    let mut ans = vec![-1; n];

    for i in (0..n).rev() {
        while let Some(&top) = stack.last() {
            if top <= a[i] {
                stack.pop();
            } else {
                ans[i] = top;
                break
            }
        }
        stack.push(a[i]);
    }

    for i in ans {
        write!(output, "{} ", i)?;
    }

    Ok(())
}