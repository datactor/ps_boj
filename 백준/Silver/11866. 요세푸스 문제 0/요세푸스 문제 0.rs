use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;
    write!(output, "<")?;

    let mut v = input.split_ascii_whitespace();

    let (n, k) =
        (v.next().unwrap().parse::<usize>().unwrap(),
         v.next().unwrap().parse::<usize>().unwrap());

    let mut deck: Vec<usize> = (1..n+1).map(|s| s).collect();

    let mut idx = k;
    while deck.len() > 1 {
        if idx <= deck.len() {
            write!(output, "{}, ", deck[idx - 1]);
            deck.remove(idx - 1);
            idx += k - 1
        } else {
            while idx > deck.len() {
                idx -= deck.len();
            }
        }
    }

    write!(output, "{}>", deck[0])?;
    Ok(())
}