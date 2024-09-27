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

    let row : Vec<i32> = input_user_to_vec();
    let n = row[0];

    let mut stack : Vec<i32> = Vec::new();

    for _ in 0..n{
        let row : Vec<i32> = input_user_to_vec();
        let n = row[0];
        if n == 0{
            stack.pop();
        }
        else{
            stack.push(n);
        }
    }
    let sum : i32 = stack.iter().sum();
    println!("{}",sum);
}
