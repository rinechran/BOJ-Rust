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
    let first_input: Vec<u32> = input_user_to_vec();
    let n = first_input[0];
    let k = first_input[1];
    let top: u32 = (n - k + 1..=n).product();
    let bottom: u32 = (1..=k).product();
    println!("{}",top/bottom);
}
