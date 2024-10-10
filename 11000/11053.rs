use std::cmp::max;
use std::io::stdin;

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
    let n = temp[0];
    let input: Vec<usize> = input_user_to_vec();

    let mut dp = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if input[j] < input[i] {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }

    let lis_length = *dp.iter().max().unwrap();
    println!("{}", lis_length);
}
