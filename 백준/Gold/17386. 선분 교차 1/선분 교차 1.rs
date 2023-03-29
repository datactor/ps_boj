use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let v = input.split_ascii_whitespace().map(|s| s.parse::<i128>().unwrap()).collect::<Vec<i128>>();

    let ccw123 = ccw((v[0], v[1]), (v[2], v[3]), (v[4], v[5]));
    let ccw124 = ccw((v[0], v[1]), (v[2], v[3]), (v[6], v[7]));
    let ccw341 = ccw((v[4], v[5]), (v[6], v[7]), (v[0], v[1]));
    let ccw342 = ccw((v[4], v[5]), (v[6], v[7]), (v[2], v[3]));

    writeln!(output, "{}",
        if ccw123 * ccw124 < 0 && ccw341 * ccw342 < 0 {
            1
        } else {
            0
        }
    )?;

    Ok(())
}

fn ccw(a: (i128, i128), b: (i128, i128), c: (i128, i128)) -> i128 {
    (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)
}