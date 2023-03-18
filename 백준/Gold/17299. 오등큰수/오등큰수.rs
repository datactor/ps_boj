use std::{
    io::{self, prelude::*, BufWriter},
    collections::HashMap,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let a: Vec<i32> = input.map(|s| s.parse::<i32>().unwrap()).collect();

    let mut freq = HashMap::new();
    for &x in &a {
        *freq.entry(x).or_insert(0) += 1;
    }

    let mut ans = vec![-1; n];
    let mut stack = Vec::new();

    for i in 0..n {
        while let Some(j) = stack.last() {
            if freq[&a[*j]] < freq[&a[i]] {
                ans[*j] = a[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    for i in ans {
        write!(output, "{} ", i)?;
    }

    Ok(())
}