use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let _: Vec<i32> = input_user_to_vec();
    let mut row: Vec<i32> = input_user_to_vec();
    let n: i32 = input_user_to_vec()[0];

    row.sort();

    if row.iter().any(|&x| x == n) {
        println!("0");
        return;
    }

    let lower_index = row.partition_point(|&x| x < n);

    let left = if lower_index == 0 { 1 } else { row[lower_index - 1] + 1 };
    let right = if lower_index == row.len() { n + 1 } else { row[lower_index] - 1 };

    let left_count = n - left;
    let right_count = right - n;

    println!("{}", left_count * (right_count + 1) + right_count);
}
