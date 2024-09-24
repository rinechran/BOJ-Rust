use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let n: i32 = input_user_to_vec()[0];
    let mut stack: Vec<i32> = Vec::new();

    for _ in 0..n {
        let row: Vec<String> = input_user_to_vec();
        let command = row[0].as_str();

        match command {
            "push" => {
                if let Ok(val) = row[1].parse::<i32>() {
                    stack.push(val);
                }
            }
            "empty" => {
                println!("{}", if stack.is_empty() { 1 } else { 0 });
            }
            "size" => {
                println!("{}", stack.len());
            }
            "top" => {
                if let Some(&top) = stack.last() {
                    println!("{}", top);
                } else {
                    println!("-1");
                }
            }
            "pop" => {
                if let Some(top) = stack.pop() {
                    println!("{}", top);
                } else {
                    println!("-1");
                }
            }
            _ => {}
        }
    }
}
