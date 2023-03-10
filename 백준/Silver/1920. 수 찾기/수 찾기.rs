use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::HashSet,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.lines();
    input.next();

    let n_list: HashSet<i32> = input.next().unwrap().split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect();

    input.skip(1).next().unwrap().split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|i| 
            writeln!(output, "{}", match n_list.contains(&i) {
                true => 1,
                false => 0,
            }).unwrap()
        );

    Ok(())
}