use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {

    let input: Vec<isize> = input_user_to_vec();
    let n = input[0];

    let mut bucket = BTreeMap::new();
    for _ in 0..n{
        let input: Vec<isize> = input_user_to_vec();
        let num = input[0];
        *bucket.entry(Reverse(num)).or_insert(0) +=1;
    }

    if let Some((&Reverse(key), _)) = bucket
        .iter()
        .max_by_key(|entry| entry.1) {
        println!("{}",key);
    }
}
