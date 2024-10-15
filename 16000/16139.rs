use std::io::stdin;

fn input_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string() 
}

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let s = input_string();
    let input: Vec<i128> = input_user_to_vec();
    let nc = input[0] as usize;

    let mut dp: Vec<Vec<isize>> = vec![vec![0; s.len()]; 26];

    for i in 0..s.len() {
        let char_index = (s.as_bytes()[i] - b'a') as usize;
        if i > 0 {
            for j in 0..26 {
                dp[j][i] = dp[j][i - 1];
            }
        }
        dp[char_index][i] += 1;
    }

    for _ in 0..nc {
        let mut input_query = String::new();
        stdin().read_line(&mut input_query).unwrap();

        let parts: Vec<&str> = input_query.trim().split_whitespace().collect();
        let character = parts[0].chars().next().unwrap();
        let left_index: usize = parts[1].parse().unwrap();
        let right_index: usize = parts[2].parse().unwrap();
        let char_index = (character as u8 - b'a') as usize;

        let result = if left_index == 0 {
            dp[char_index][right_index]
        } else {
            dp[char_index][right_index] - dp[char_index][left_index - 1]
        };

        println!("{}", result);
    }
}
