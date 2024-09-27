use std::io;
fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let stdin = io::stdin();
    let mut input_string = String::new();
    stdin.read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn main() {


    loop{
        let row : Vec<String> = input_user_to_vec();
 
    }
}
