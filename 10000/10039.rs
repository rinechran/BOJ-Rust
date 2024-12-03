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

    let mut array = Vec::new();

    for _ in 0..5 {
        let row: Vec<usize> = input_user_to_vec();
        let store = row[0].max(40);
        array.push(store);
    }
    let total : usize = array.iter().sum();
    println!("{}",total/5);
}
