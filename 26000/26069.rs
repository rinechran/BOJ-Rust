use std::backtrace::Backtrace;
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
    let tc = first_input[0];

    let mut dancing_set: HashSet<String> = HashSet::new();
    dancing_set.insert("ChongChong".to_string());

    for _ in 0..tc{
        let input: Vec<String> = input_user_to_vec();

        if dancing_set.contains(&input[0]) == true || dancing_set.contains(&input[1]) == true  {
            dancing_set.insert(input[0].to_string());
            dancing_set.insert(input[1].to_string());
        }
    }
    println!("{}",dancing_set.len());

}