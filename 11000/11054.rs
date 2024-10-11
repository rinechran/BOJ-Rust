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
    let temp: Vec<usize> = input_user_to_vec();
    let input: Vec<usize> = input_user_to_vec();
    let count = input.len();
    let mut inc_dp : Vec<usize> = vec![1;count];
    let mut des_dp : Vec<usize> = vec![1;count];

    for i in  0..count{
        for j in 0..i{
            if input[j]< input[i]{
                inc_dp[i] = max(inc_dp[i], inc_dp[j] + 1);
            }
        }
    }
    for i in (0..count).rev() {
        for j in i..count {
            if input[j] < input[i] {
                des_dp[i] = max(des_dp[i], des_dp[j] + 1);
            }
        }
    }

    let mut max_len = 0;
    for i in 0..count {
        max_len = max(max_len, inc_dp[i] + des_dp[i] - 1);
    }
    println!("{}", max_len);


}
