use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn array(x: isize, y: isize) -> isize {
    let layer = x.abs().max(y.abs());
    let quarter = layer << 1;
    let cells = (quarter - 1).pow(2);
    match x {
        x if x == layer && y == layer => ((layer + 1 << 1) - 1).pow(2),
        x if x == layer => cells + (layer - y),
        x if y == -layer => cells + quarter + (layer - x),
        x if x == -layer => cells + 3 * quarter + (y - layer),
        x if y == layer => cells + 4 * quarter + (x - layer),
        _ => {layer},
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input).unwrap();

    let mut v = input.split_ascii_whitespace().map(
        |s| s.parse::<isize>()).flatten();
    let (r1, c1, r2, c2) =
    (v.next().unwrap(), v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
    let max = std::cmp::max(
        array(c1, r1).max(array(c1, r2)),
        array(c2, r1).max(array(c2, r2)),
    );
    let len = max.to_string().len();
    for r in r1..=r2 {
        for c in c1..=c2 {
            write!(output, "{1:0$} ", len, array(c, r))?;
        }
        writeln!(output)?;
    }

    Ok(())
}