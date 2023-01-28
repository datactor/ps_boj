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

    loop {
        let mut v = lines.next().unwrap().split_ascii_whitespace()
            .map(|s| s.parse::<usize>()).flatten();
        let n = v.next().unwrap();
        if n == 0 {
            break
        }
        let x: Vec<usize> = v.collect();
        let mut answer = 0;

        let mut stack = VecDeque::new();
        let mut width = 0;
        for i in 0..n {
            while !stack.is_empty() && x[*stack.back().unwrap()] > x[i] {
                let tmp = stack.pop_back().unwrap();

                if stack.is_empty() {
                    width = i;
                } else {
                    width = i - stack.back().unwrap() - 1;
                }

                answer = usize::max(answer, width * x[tmp]);
            }
            stack.push_back(i)
        }
        while !stack.is_empty() {
            let tmp = stack.pop_back().unwrap();
            
            if stack.is_empty() {
                width = n;
            } else {
                width = n - stack.back().unwrap() - 1;
            } answer = usize::max(answer, width * x[tmp])
        }

        writeln!(output, "{}", answer)?;
    }

    Ok(())
}