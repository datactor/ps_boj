use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let s = lines.next().unwrap().split_ascii_whitespace().map(|s| s.as_bytes()).collect::<Vec<_>>();

    for line in lines.skip(1) {
        let mut v = line.split_ascii_whitespace().map(|s| s.parse::<String>()).flatten();

        let a = v.next().unwrap().as_bytes()[0];
        let l = v.next().unwrap().parse::<usize>().unwrap();
        let r = v.next().unwrap().parse::<usize>().unwrap();

        let x = &s[0][l..r+1].iter().filter(|&s| s == &a).count();
        writeln!(output, "{}", x)?;
    }
    
    Ok(())
}