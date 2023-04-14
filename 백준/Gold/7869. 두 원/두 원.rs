use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let line = input.split_ascii_whitespace().map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    let (x1, y1, r1, x2, y2, r2) = (line[0], line[1], line[2], line[3], line[4], line[5]);
    let d = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();

    writeln!(output, "{:.3}",
             if r1 + r2 <= d {
                 0.0
             } else if (r1 - r2).abs() >= d {
                 std::f32::consts::PI * r1.min(r2).powf(2.0)
             } else {
                 let theta1 = ((r1.powf(2.0) + d.powf(2.0) - r2.powf(2.0)) / (2.0 * r1 * d)).acos();
                 let theta2 = ((r2.powf(2.0) + d.powf(2.0) - r1.powf(2.0)) / (2.0 * r2 * d)).acos();

                 let s1 = r1.powf(2.0) * theta1 - r1.powf(2.0) * (2.0 * theta1).sin() / 2.0;
                 let s2 = r2.powf(2.0) * theta2 - r2.powf(2.0) * (2.0 * theta2).sin() / 2.0;
                 s1 + s2
             }
    )?;

    Ok(())
}