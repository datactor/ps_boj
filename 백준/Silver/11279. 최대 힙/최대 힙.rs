use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::BinaryHeap,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace();
    input.next();
    
    let mut q = BinaryHeap::new();

    for i in input {
        let x = i.parse::<usize>().unwrap();

        match x {
            0 => writeln!(output, "{}", q.pop().unwrap_or(0))?,
            _ => q.push(x),
        }
    }

    Ok(())
}