use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let line = input.split_ascii_whitespace().map(|s| s.parse::<f64>().unwrap()).collect::<Vec<f64>>();

    let (x, y, d, t) = (line[0], line[1], line[2], line[3]);

    let mut dist = (x.powf(2.0) + y.powf(2.0)).sqrt();

    writeln!(output, "{:.9}",
             if d <= t {
                 dist
             } else {
                 let jump = (dist / d).floor();
                 let mut min_time = dist;

                 dist -= jump * d;

                 if jump == 0.0 {
                     min_time = min_time.min(f64::min(t + d - dist, 2.0 * t));
                 } else {
                     min_time = min_time.min(f64::min(jump * t + dist, (jump + 1.0) * t));
                 }
                 min_time
             }
    )?;

    Ok(())
}