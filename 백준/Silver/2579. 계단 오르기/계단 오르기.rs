use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().skip(1).map(
        |s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut arr = vec![0; v.len()];

    if v.len() == 1 {
        writeln!(output, "{}", v[0]).unwrap();
    } else if v.len() == 2 {
        writeln!(output, "{}", v[..2].iter().sum::<usize>()).unwrap();
    } else {
        arr[0] = v[0];
        arr[1] = v[0] + v[1];
        for i in 2..v.len() {
            if i == 2 {
                arr[i] = (arr[i-2] + v[i]).max(v[i-1]+v[i])
            } else {
                arr[i] = (arr[i-2] + v[i]).max(arr[i-3]+v[i-1]+v[i])
            }
        }
        writeln!(output, "{}", arr[v.len()-1]).unwrap();
    }
}