use std::collections::HashSet;
use std::io::{self, stdin};
use std::iter::FromIterator;

fn input_user_to_vec<T: std::str::FromStr, K: FromIterator<T>>() -> K {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let nums: Vec<usize> = input_user_to_vec();
    let (n, m) = (nums[0], nums[1]);

    let mut s_set: HashSet<String> = HashSet::with_capacity(n);

    for _ in 0..n {
        let line: Vec<String> = input_user_to_vec();
        s_set.insert(line[0].clone());
    }

    let mut count = 0;

    for _ in 0..m {
        let line: Vec<String> = input_user_to_vec();
        if s_set.contains(&line[0]) {
            count += 1;
        }
    }

    println!("{}", count);
}
