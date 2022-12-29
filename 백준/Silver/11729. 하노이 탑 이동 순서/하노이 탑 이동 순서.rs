use std::io::stdin;
use std::fmt::Write;

fn hanoi(n: usize, s: usize, e: usize, output: &mut String) {
    if n == 1 {
        writeln!(output, "{} {}", s, e);
        return
    }
    hanoi(n - 1, s, 6 - s -  e, output);
    writeln!(output, "{} {}", s, e);
    hanoi(n-1, 6 - s - e, e, output);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();
    let mut output = String::new();
    writeln!(output, "{}", i32::pow(2, n as u32) - 1);
    hanoi(n, 1, 3, &mut output);
    println!("{}", output);
}