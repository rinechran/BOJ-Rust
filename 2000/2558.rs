use std::io::{self, Read, stdin};

fn input_user_to_vec<T : std::str::FromStr>() -> Vec<T>{
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn main(){
    let A : i32= input_user_to_vec()[0];
    let B : i32 = input_user_to_vec()[0];
    println!("{}",A+B);
}