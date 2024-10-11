use std::cmp::max;
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
    let input: Vec<usize> = input_user_to_vec();
    let n = input[0];
    let k = input[1];
    let mut dp = vec![vec![0;k +1];n +1 ];

    let mut weight_value = Vec::new();
    for _ in 0..n{
        let input: Vec<usize> = input_user_to_vec();
        weight_value.push((input[0],input[1]));
    }

    for i in 1..=n {
        let (weight, value) = weight_value[i - 1];
        for j in 0..=k {
            if weight <= j {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - weight] + value);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    println!("{}", dp[n][k]);

}
