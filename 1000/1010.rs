use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn factorial(mut n : u128 ) -> u128{

    let mut result = 1;
    for i in 1..n+1{
        result = result *i;
    }

    return result
}
fn main() {
    let n = input_user_to_vec()[0];
    for _ in 0..n {
        let input = input_user_to_vec();
        let numerator = factorial(input[1]);
        let denominator = factorial(input[0]) * factorial(input[1]-input[0]);
        println!("{}",numerator/denominator);

    }
}