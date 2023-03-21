use std::io::{self, prelude::*, BufWriter};

const MAX: usize = 1_000_000;

fn main() -> io::Result<()> {
    let mut is_prime_list = [false; 1_000_000];

    (is_prime_list[0], is_prime_list[1], is_prime_list[2]) = (true, true, true);

    for i in 2..(MAX as f32).sqrt() as usize {
        if is_prime_list[i] == false {
            for j in (i * i..MAX).step_by(i) {
                is_prime_list[j] = true;
            }
        }
    }

    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap().trim().parse::<usize>().unwrap()); // lazy
    let mut output = BufWriter::new(io::stdout().lock());

    while let Some(n) = input.next() {
        if n == 0 {
            break;
        }

        let mut left = 3;
        let mut right = n - 3;

        while left <= right {
            if !is_prime_list[left] && !is_prime_list[right] {
                if left + right == n {
                    break;
                }
            }
            left += 2;
            right -= 2;
        }

        if left > right {
            writeln!(output, "Goldbach's conjecture is wrong.")?;
        } else {
            writeln!(output, "{} = {} + {}", n, left, right)?;
        }
    }

    Ok(())
}