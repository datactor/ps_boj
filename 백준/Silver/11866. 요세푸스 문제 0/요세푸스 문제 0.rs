use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;
    write!(output, "<")?;

    let mut v = input.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap());
    let (n, k) =
        (v.next().unwrap(),
         v.next().unwrap());

    let mut m: Vec<usize> = (1..n+1).map(|s| s).collect();

    let mut idx = k;
    while m.len() > 1 {
        if idx <= m.len() {
            write!(output, "{}, ", m[idx-1])?;
            m.remove(idx-1);
            idx += k-1
        } else {
            match idx % m.len() {
                0 => idx = m.len(),
                _ => idx %= m.len(),
            }
        }
    }
    write!(output, "{}>", m[0])?;
    Ok(())
}