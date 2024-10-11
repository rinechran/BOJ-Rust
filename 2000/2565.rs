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
    let mut input_arr : Vec<(usize,usize)> = Vec::new();
    let mut dp : Vec<usize> = vec![0;n];
    for _ in 0..n{
        let input: Vec<usize> = input_user_to_vec();
        input_arr.push((input[0],input[1]));
    }
    input_arr.sort_by(|a,b| a.0.cmp(&b.0));

    let mut dp : Vec<usize> = vec![1;n];

    for i in 0..n{
        for j in 0..i{
            if input_arr[j].1<input_arr[i].1{
                dp[i] = max(dp[i] , dp[j]+1);
            }
        }
    }
    let lis_length  = *dp.iter().max().unwrap();
    println!("{}",dp.len()-lis_length);

}
