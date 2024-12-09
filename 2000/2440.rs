use std::collections::{HashMap, VecDeque};
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let count = row[0];
    for i in (1..=count).rev(){
        for _ in 0..i{
            print!("*");
        }
        println!();
    }
}
