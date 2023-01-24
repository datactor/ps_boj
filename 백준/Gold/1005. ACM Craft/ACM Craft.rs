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

    let t = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut f_line = lines.next().unwrap().split_ascii_whitespace();
        let (n, k) = (f_line.next().unwrap().parse::<usize>().unwrap(),
            f_line.next().unwrap().parse::<usize>().unwrap());
        let build_times: Vec<usize> = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>().unwrap()).collect();

        let mut indegree = vec![0; n+1];
        let mut seq = vec![Vec::new(); n+1];

        for _ in 0..k {
            let mut tmp = lines.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>()).flatten();
            let (x, y) = (tmp.next().unwrap(), tmp.next().unwrap());
            seq[x].push(y);
            indegree[y] += 1;
        }

        let building_num = lines.next().unwrap().parse::<usize>().unwrap();

        let mut dp = vec![0; n+1];
        let mut q = VecDeque::from(vec![0; n+1]);

        for i in 1..n+1 {
            if indegree[i] == 0 {
                q.push_back(i);
                dp[i] = build_times[i-1];
            }
        }

        while !q.is_empty() {
            let a = q.pop_front().unwrap();
            for i in &seq[a] {
                indegree[*i] -= 1;
                dp[*i] = std::cmp::max(dp[a] + build_times[*i-1], dp[*i]);
                if indegree[*i] == 0 {
                    q.push_back(*i);
                }
            }
        }

        writeln!(output, "{}", dp[building_num])?;
    }

    Ok(())
}