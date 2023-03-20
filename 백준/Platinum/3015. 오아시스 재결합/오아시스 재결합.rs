use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let heights: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|p| p.parse::<usize>().unwrap())
        .collect();

    let mut stack = Vec::new();
    let mut ans = 0;

    for h in heights {
        while let Some((prev_h, prev_cnt)) = stack.pop() {
            if prev_h < h {
                ans += prev_cnt as u64;
            } else {
                stack.push((prev_h, prev_cnt));
                break;
            }
        }

        let mut cnt = 1;
        if let Some((top, top_cnt)) = stack.last_mut() {
            if *top == h {
                cnt += *top_cnt;
                ans += *top_cnt as u64;
                stack.pop();
            }
        }
        if !stack.is_empty() {
            ans += 1;
        }
        stack.push((h, cnt));
    }

    writeln!(output, "{}", ans)?;
    Ok(())
}