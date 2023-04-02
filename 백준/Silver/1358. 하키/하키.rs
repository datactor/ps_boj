use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());
    let mut line = || input.next().unwrap().unwrap();
    let mut line_to_vec = || line().split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let first_line = line_to_vec();
    let (w, h, x, y, p) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

    let mut cnt = 0;
    for _ in 0..p as usize {
        let pos = line_to_vec();

        if x <= pos[0] && pos[0] <= x+w && y <= pos[1] && pos[1] <= y+h {
            cnt += 1;
            continue
        }

        let dist_from_front_sq = (pos[0] - x).pow(2) + (pos[1] - (y+h/2)).pow(2);
        let dist_from_back_sq = (pos[0] - (x+w)).pow(2) + (pos[1] - (y+h/2)).pow(2);

        if dist_from_back_sq <= (h / 2).pow(2) || dist_from_front_sq <= (h / 2).pow(2) {
            cnt += 1
        }
    }

    writeln!(output, "{}", cnt)?;

    Ok(())
}