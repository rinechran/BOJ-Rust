use std::collections::{HashSet, VecDeque};
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

    let mut set = HashSet::new();
    let mut count = 0;
    for _ in 0..n {
        let input: Vec<String> = input_user_to_vec();
        let input = input[0].clone();
        match input.as_str() {
            "ENTER" =>{
                count += set.len();
                set.clear();
            }
            _ =>{
                set.insert(input);
            }
        }
    }

    count += set.len();

    println!("{}",count);

}