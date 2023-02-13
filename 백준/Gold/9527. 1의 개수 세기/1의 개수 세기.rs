use std::{
    io,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut input = input.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap());
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", check_bin1(b) - check_bin1(a-1));

    Ok(())
}

fn check_bin1(mut n: u64) -> u64 {
    let mut psum = [0; 58];
    (1..58).for_each(|i| psum[i] = 2_u64.pow(i as u32 - 1) + 2 * psum[i-1]);

    let mut cnt = 0;
    let bin_num = format!("{:b}", n).as_bytes().iter().map(|&s| s-48).collect::<Vec<u8>>();
    let len = bin_num.len();

    for i in 0..len {
        if bin_num[i] == 1 {
            let pow = len - i - 1;
            cnt += psum[pow];
            n -= 2_u64.pow(pow as u32);
            cnt += n + 1;
        }
    } cnt
}