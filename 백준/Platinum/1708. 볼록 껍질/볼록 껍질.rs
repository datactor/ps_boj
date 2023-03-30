use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut points = input.skip(1).map(|line|
        {
            let string = line.unwrap();
            let (x, y) = string.split_once(' ').unwrap();
            let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
            (x, y)
        }
    ).collect::<Vec<(i32, i32)>>();

    points.sort_by(|a, b| a.cmp(b));

    let mut result = convex_hull(&mut points);
    points.reverse();
    result += convex_hull(&mut points);

    writeln!(output, "{}", result - 2)?;

    Ok(())
}

fn ccw(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> bool {
    (x2 as i64 - x1 as i64) * (y3 as i64 - y1 as i64) - (y2 as i64 - y1 as i64) * (x3 as i64 - x1 as i64) > 0
}

fn convex_hull(points: &mut Vec<(i32, i32)>) -> usize {
    let mut hull: Vec<(i32, i32)> = Vec::new();
    for &p3 in points.iter() {
        while hull.len() >= 2 {
            let p1 = hull[hull.len() - 2];
            let p2 = hull[hull.len() - 1];
            if ccw(p1.0, p1.1, p2.0, p2.1, p3.0, p3.1) {
                break;
            }
            hull.pop();
        }
        hull.push(p3);
    }
    return hull.len();
}