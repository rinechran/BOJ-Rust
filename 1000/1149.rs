use std::cmp;
use std::cmp::{max, min};
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
    let nc : Vec<usize> = input_user_to_vec();
    let nc = nc[0];
    let mut wall_colors_weight: Vec<Vec<usize>> = Vec::new();
    for _ in 0..nc{
        let temp : Vec<usize> = input_user_to_vec();
        wall_colors_weight.push(temp);
    }

    let mut dp  = vec![vec![0;3];nc];

    dp[0] = wall_colors_weight[0].clone();
    for i in 1..nc{
        dp[i][0] = wall_colors_weight[i][0] + min(dp[i-1][1],dp[i-1][2]);
        dp[i][1] = wall_colors_weight[i][1] + min(dp[i-1][0],dp[i-1][2]);
        dp[i][2] = wall_colors_weight[i][2] + min(dp[i-1][0],dp[i-1][1]);
    }

    let min_cost = dp[nc-1].iter().min().unwrap();
    println!("{}",min_cost);


}
