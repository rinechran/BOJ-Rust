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
    let mut notes: Vec<usize> = input_user_to_vec();

    if notes == vec![1, 2, 3, 4, 5, 6, 7, 8] {
        println!("ascending");
    } else if notes == vec![8, 7, 6, 5, 4, 3, 2, 1] {
        println!("descending");
    } else {
        println!("mixed");
    }
}
