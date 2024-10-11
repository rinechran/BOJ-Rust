use std::cmp::max;
use std::collections::VecDeque;
use std::io::stdin;

fn input_user_to_vec() -> VecDeque<char> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .trim()
        .chars()
        .collect()
}

fn main() {
    let mut input1: VecDeque<char> = input_user_to_vec();
    let mut input2: VecDeque<char> = input_user_to_vec();
    input1.push_front('0'); // 패딩 추가
    input2.push_front('0'); // 패딩 추가

    let mut dp = vec![vec![0; input2.len()]; input1.len()];
    let mut max_value = 0;

    for i in 1..input1.len() {
        for j in 1..input2.len() {
            if input1[i] == input2[j] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i][j-1] , dp[i-1][j]);
            }
        }
    }

    println!("{}", dp[input1.len()-1][input2.len()-1]);
}
