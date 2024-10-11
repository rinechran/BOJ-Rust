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
    let total_day : usize = input[1] ;

    let row: Vec<i32> = input_user_to_vec();
    let mut dp : Vec<i32> = vec![0; input[0]] ;

    let mut temp_total = 0;
    for i in 0..total_day {
        temp_total += row[i];
    }
    dp[total_day-1] = temp_total;

    let mut max_value = temp_total;
    for i in total_day..row.len(){
        dp[i] =  dp[i - 1] + row[i] - row[i - total_day];
        max_value = max(max_value,dp[i]);
    }
    println!("{}",max_value);


}
