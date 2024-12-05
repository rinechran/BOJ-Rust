use std::collections::{VecDeque, HashMap};
use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn count_less_equal(n : usize , x:usize) -> usize{
    let mut count = 0;
    for i in 1..=n{
        count+=(x / i).min(n);
    }
    return count;
}
fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let n = row[0];
    let row: Vec<usize> = input_user_to_vec();
    let k = row[0];

    let mut left = 1;
    let mut right = n*n;
    let mut answer = 0;
    while left <= right {
        let mid = (left + right) / 2;
        if count_less_equal(n, mid) >= k {
            answer = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }

    }
    println!("{}",answer);
}
