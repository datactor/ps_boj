use std::{error::Error, io};

fn nqueen(n: usize, map: &mut Vec<usize>) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    for i in 0..map.len() {
        if map[i] != 0 {
            continue;
        }

        let check = map
            .iter()
            .enumerate()
            .filter(|(_, &val)| val != 0)
            .map(|(i, &val)| (i as i32 - val as i32, i + val))
            .any(|(x, y)| x == i as i32 - n as i32 || y == i + n);
        if check == false {
            map[i] = n;
            count += nqueen(n - 1, map);
            map[i] = 0;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut map = vec![0; n];
    let ret = nqueen(n, &mut map);

    println!("{ret}");

    Ok(())
}