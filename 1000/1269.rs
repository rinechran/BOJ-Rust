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
    let temp: Vec<usize> = input_user_to_vec();
    let input_set_1: Vec<usize> = input_user_to_vec();
    let input_set_2: Vec<usize> = input_user_to_vec();

    let set_1 : HashSet<_> = input_set_1.iter().clone().collect();
    let set_2 : HashSet<_> = input_set_2.iter().clone().collect();

    let difference1: HashSet<_> = set_1.difference(&set_2).collect();
    let difference2: HashSet<_> = set_2.difference(&set_1).collect();

    println!("{}",difference1.len() + difference2.len());

}
