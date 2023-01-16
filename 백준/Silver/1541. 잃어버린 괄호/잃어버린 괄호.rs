use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut sum = 0;
    let mut idx = 0;
    let mut first_minus = false;
    for i in 0..input.len() {
        if !first_minus {
            match &input[i..i+1] {
                "-" => {
                    sum += input[idx..i].parse::<i32>().unwrap();
                    idx = i+1;
                    first_minus = true;
                },
                "+" => {
                    sum += input[idx..i].parse::<i32>().unwrap();
                    idx = i+1;
                },
                "\n" => sum += input[idx..i].parse::<i32>().unwrap(),
                _ => {},
            }
        } else {
            if &input[i..i+1] == "-" || &input[i..i+1] == "+" {
                sum -= input[idx..i].parse::<i32>().unwrap();
                idx = i+1;
            } else if &input[i..i+1] == "\n" {
                sum -= input[idx..i].parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", sum);
    Ok(())
}