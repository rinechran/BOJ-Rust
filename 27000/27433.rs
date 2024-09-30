use std::collections::VecDeque;
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
    let first_input: Vec<u128> = input_user_to_vec();
    let n = first_input[0];
    let top: u128 = (1..=n).product();
    println!("{}",top);
}