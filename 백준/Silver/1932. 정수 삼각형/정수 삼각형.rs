use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let f = lines.next().unwrap().parse::<usize>().unwrap();

    let mut arr = vec![f];

    for i in 1..n {
        let mut tmp = Vec::new();
        let v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        for j in 0..v.len() {
            if j == 0 {
                tmp.push(v[j] + arr[j])
            } else if j == v.len() - 1 {
                tmp.push(v[j] + arr[j-1])
            } else {
                tmp.push((v[j] + arr[j-1]).max(v[j] + arr[j]));
            }
        }
        arr = tmp;
    }
    writeln!(output, "{}", arr.iter().max().unwrap()).unwrap();
}