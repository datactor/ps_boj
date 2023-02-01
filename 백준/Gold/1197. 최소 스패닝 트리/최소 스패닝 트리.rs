use std::{
    io::{self, prelude::*},
    error::Error,
    collections::BinaryHeap,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut read = || input.next().unwrap();

    let (v, e) = (read(), read());

    let mut graph = vec![vec![]; (v + 1) as usize];
    (0..e).for_each(|_| {
        let (u, v, w) = (read(), read(), read());
        graph[u as usize].push((v, w));
        graph[v as usize].push((u, w));
    });

    println!("{}", prim(1, 0, graph, v as usize));

    Ok(())
}

fn prim(start: usize, weight: i32, graph: Vec<Vec<(i32, i32)>>, n: usize) -> i32 {
    let mut visited = vec![0; n+1];
    let mut q = BinaryHeap::from([(weight, start)]);
    let mut sum = 0;
    let mut cnt = 0;

    while cnt < n {
        let (k, v) = q.pop().unwrap();
        if visited[v] == 1 {
            continue
        }
        visited[v] = 1;
        sum -= k;
        cnt += 1;
        graph[v].iter().for_each(|&(u, w)| q.push((-w, u as usize)));
    } sum
}