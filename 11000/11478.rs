use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn input_user_to_vec<T: std::str::FromStr, K: FromIterator<T>>() -> K {
    let stdin = io::stdin();
    let mut input_string = String::new();
    stdin.read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let temp: Vec<String> = input_user_to_vec();
    let input_str = &temp[0];

    let mut substrings: HashSet<String> = HashSet::new();
    for i in 0..input_str.len() {
        for j in i + 1..=input_str.len() {
            substrings.insert(input_str[i..j].to_string());
        }
    }
    println!("{}", substrings.len());
}
