use std::{
    io::{self, prelude::*},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let v: Vec<_> = input.split_ascii_whitespace().map(
        |s| s.as_bytes()).collect();

    let mut arr = vec![0; v[1].len()];

    for i in 0..v[0].len() {
        let mut idx = 0;
        for j in 0..v[1].len() {
            if idx < arr[j] {
                idx = arr[j]
            } else if v[0][i] == v[1][j] {
                arr[j] = idx + 1
            }
        }
    }
    
    println!("{}", arr.iter().max().unwrap());
    /////////////////////////////
    /////////////////////////////
    /////////////////////////////
    /////////////////////////////
    /////////////////////////////
    /////////////////////////////
    /////////////////////////////

    Ok(())
}