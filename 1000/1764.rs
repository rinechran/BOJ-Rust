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
    let nums: Vec<usize> = input_user_to_vec();
    let n = nums[0];
    let m = nums[1];

    let mut unheard: HashSet<String> = HashSet::with_capacity(n);
    for _ in 0..n {
        let name : Vec<String>= input_user_to_vec();
        unheard.insert(name[0].clone());
    }
    let mut unheard_and_unseen: Vec<String> = Vec::new();

    for _ in 0..m {
        let row : Vec<String>= input_user_to_vec();

        if unheard.contains(row[0].as_str()) {
            unheard_and_unseen.push(row[0].clone());
        }
    }

    unheard_and_unseen.sort();

    println!("{}", unheard_and_unseen.len());
    for name in unheard_and_unseen {
        println!("{}", name);
    }
}
