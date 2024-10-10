use std::cmp;
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
    let nc : Vec<usize> = input_user_to_vec();
    let nc = nc[0];
    let mut stars : Vec<usize> = Vec::new();
    stars.push(0);
    for _ in 0..nc{
        let temp : Vec<usize> = input_user_to_vec();
        stars.push(temp[0]);
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; nc+1];
    dp[1][0] = stars[1];
    dp[1][1] = stars[1];
    for i in 2..=nc {
        dp[i][0] = dp[i - 1][1] + stars[i];
        dp[i][1] = max(dp[i - 2][0], dp[i - 2][1]) + stars[i];
    }

    println!("{}", max(dp[nc][0], dp[nc][1]));

}
