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
    let mut wine_weight: Vec<usize> = Vec::new();
    for _ in 0..nc{
        let temp : Vec<usize> = input_user_to_vec();
        wine_weight.push(temp[0]);
    }

    if nc == 1 {
        println!("{}", wine_weight[0]);
        return;
    } else if nc == 2 {
        println!("{}", wine_weight[0] + wine_weight[1]);
        return;
    }

    let mut dp  = vec![0;nc];
    dp[0] = wine_weight[0];
    dp[1] = wine_weight[0] + wine_weight[1];
    dp[2] = max(
        wine_weight[0] + wine_weight[2],
        max(wine_weight[1] + wine_weight[2], dp[1]),
    );

    for i in 3..nc{
        dp[i] = max(
            dp[i-1],
            max(dp[i - 2] + wine_weight[i], dp[i - 3] + wine_weight[i - 1] + wine_weight[i]),
        )
    }
    println!("{}",dp[nc-1]);

}
