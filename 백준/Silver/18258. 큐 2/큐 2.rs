use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut q = VecDeque::new();

    for _ in 0..n {
        let mut line = lines.next().unwrap().split_ascii_whitespace();
        match line.next().unwrap() {
            "push" => q.push_back(line.next().unwrap().parse::<i32>().unwrap()),
            "pop" => if q.is_empty() { writeln!(output, "-1").unwrap() } else { writeln!(output, "{}", q.pop_front().unwrap()).unwrap() },
            "size" => writeln!(output, "{}", q.len()).unwrap(),
            "empty" => if q.is_empty() { writeln!(output, "1").unwrap() } else { writeln!(output, "0").unwrap() },
            "front" => if q.is_empty() { writeln!(output, "-1").unwrap() } else { writeln!(output, "{}", q[0]).unwrap() },
            "back" => if q.is_empty() { writeln!(output, "-1").unwrap() } else { writeln!(output, "{}", q[q.len()-1]).unwrap() },
            _ => {},
        }
    }

    Ok(())
}