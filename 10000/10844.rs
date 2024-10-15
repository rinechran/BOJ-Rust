use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn find(val: usize) -> i128 {
    let mut arr = vec![vec![0; 10]; 101];

    for j in 1..=9 {
        arr[1][j] = 1;
    }

    for i in 2..=val {
        for j in 0..=9 {
            if j > 0 {
                arr[i][j] += arr[i - 1][j - 1];
            }
            if j < 9 {
                arr[i][j] += arr[i - 1][j + 1];
            }
            arr[i][j] %= 1_000_000_000;
        }
    }

    let total: i128 = arr[val].iter().sum();
    total % 1_000_000_000
}

fn main() {
    let input: Vec<usize> = input_user_to_vec();
    let nc = input[0];

    let result = find(nc);
    println!("{}", result);
}
