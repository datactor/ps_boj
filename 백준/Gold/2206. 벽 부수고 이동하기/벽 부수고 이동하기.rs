use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut line = || input.next();
    let first_line = line().unwrap().unwrap();
    let (n, m) = first_line.split_once(' ').unwrap();
    let (n, m) = (n.parse::<usize>().unwrap(), m.parse::<usize>().unwrap());

    let mut grid = input.take(n).map(|line| line.unwrap().as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut visited = vec![vec![[0; 2]; m]; n];

    writeln!(output, "{}", bfs(&mut grid, &mut visited, n, m))?;
    Ok(())
}

fn bfs(grid: &mut Vec<Vec<u8>>, visited: &mut Vec<Vec<[i32; 2]>>, n: usize, m: usize) -> i32 {
    let mut dq = VecDeque::new();
    dq.push_back((0, 0, 0));
    visited[0][0][0] = 1;

    while let Some((x, y, w)) = dq.pop_front() {
        if x == n - 1 && y == m - 1 {
            return visited[x][y][w] as i32;
        }

        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == b'0' && visited[nx][ny][w] == 0 {
                    visited[nx][ny][w] = visited[x][y][w] + 1;
                    dq.push_back((nx, ny, w));
                } else if grid[nx][ny] == b'1' && w == 0 {
                    visited[nx][ny][w + 1] = visited[x][y][w] + 1;
                    dq.push_back((nx, ny, w + 1));
                }
            }
        }
    }

    return -1;
}
