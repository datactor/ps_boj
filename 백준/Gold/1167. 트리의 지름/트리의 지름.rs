use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace();
    let mut read = || input.next().unwrap().parse::<usize>();

    let v: usize = read().unwrap();
    let mut vec = vec![vec![]; v+1];
    vec.push(vec![(0, 0)]);

    for _ in 0..v {
        let i = read().unwrap();
        loop {
            let x = if let Ok(t) = read() { t } else { break };
            vec[i].push((x, read().unwrap()));
        }
    }

    let mut visited = vec![-1; v+1];
    visited[1] = 0;
    dfs(1, 0, &vec, &mut visited);

    let max = visited.iter().max().unwrap();

    let start = visited.iter().position(|x| x == max).unwrap();
    visited = vec![-1; v+1];
    visited[start] = 0;
    dfs(start, 0, &vec, &mut visited);

    println!("{}", visited.iter().max().unwrap());
    Ok(())
}

fn dfs(idx: usize, v: usize, vec: &Vec<Vec<(usize, usize)>>, visited: &mut Vec<i32>) {
    for (node, dist) in &vec[idx] {
        if visited[*node] == -1 {
            visited[*node] = (dist + v) as i32;
            dfs(*node, dist + v, vec, visited);
        }
    }
}