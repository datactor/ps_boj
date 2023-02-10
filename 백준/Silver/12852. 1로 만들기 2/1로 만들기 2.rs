use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::HashMap,
};

struct Dp {
    inner: HashMap<i32, (i32, Vec<i32>)>,
}

impl Dp {
    fn new() -> Dp {
        Dp {
            inner: HashMap::from([
                (1, (0, vec![1])),
                (2, (1, vec![1, 2])),
                (3, (1, vec![1, 3])),
            ]
            )
        }
    }

    fn to_one(&mut self, n: i32) -> (i32, Vec<i32>) {
        if let Some(inner) = self.inner.get(&n) {
            return (*inner).clone();
        }

        let mut vec = Vec::new();
        let (mut cnt, mut quo) = (0, Vec::new());

        if n % 3 == 0 {
            (cnt, quo) = self.to_one(n / 3);
            vec.push((cnt + 1, quo));
        }
        if n % 2 == 0 {
            (cnt, quo) = self.to_one(n / 2);
            vec.push((cnt + 1, quo));
        }
        if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
            (cnt, quo) = self.to_one(n - 1);
            vec.push((cnt + 1, quo));
        }
        if (n - 2) % 3 == 0 {
            (cnt, quo) = self.to_one(n - 2);
            quo.push(n - 1);
            vec.push((cnt + 2, quo));
        }

        let (count, mut quo) = vec.iter().min().unwrap().clone();

        quo.push(n);
        self.inner.insert(n, (count, quo.clone()));

        (count, quo)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;

    let n: i32 = input.trim().parse()?;
    let mut dp = Dp::new();

    let (cnt, quo) = dp.to_one(n);

    writeln!(output, "{}", cnt)?;
    for p in quo.iter().rev() {
        write!(output, "{} ", p)?;
    }

    Ok(())
}