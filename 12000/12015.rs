use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn find_position(arr: &Vec<usize>, val: usize) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < val {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    let _: Vec<usize> = input_user_to_vec();
    let array: Vec<usize> = input_user_to_vec();

    let mut lis = Vec::new();

    for &val in &array {
        let pos = find_position(&lis, val);
        if pos < lis.len() {
            lis[pos] = val;
        } else {
            lis.push(val);
        }
    }

    println!("{}", lis.len());
}
