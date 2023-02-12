use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let strings: Vec<_> = input.split_ascii_whitespace().map(|s| s.as_bytes()).collect();

    let mut lens = vec![(0, vec![]); strings[0].len()];

    for i in 0..strings[1].len() {
        let mut cnt = (0, vec![]);
        for j in 0..strings[0].len() {
            if cnt.0 < lens[j].0 {
                cnt = lens[j].clone()
            } else if strings[1][i] == strings[0][j] {
                let mut tmp = cnt.clone();
                lens[j].0 = tmp.0 + 1;
                tmp.1.push(strings[0][j]);
                lens[j].1 = tmp.1;
            }
        }
    }
    let ans = lens.iter().max().unwrap();
    writeln!(output, "{}", ans.0)?;
    writeln!(output, "{}", std::str::from_utf8(&ans.1).unwrap())?;

    Ok(())
}