use std::collections::HashMap;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn fibonacci(n : i32) -> i32{
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}
fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];
    println!("{}",fibonacci(n));
}
