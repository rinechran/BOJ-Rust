use std::io;
use std::io::stdin;

fn inputUserToVecI32() -> Vec<i32> {
    let mut inputString = String::new();
    io::stdin()
        .read_line(&mut inputString)
        .unwrap();

    let number = inputString
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    number
}
fn main() {
    let total : i32= inputUserToVecI32()
        .iter()
        .sum();
    println!("{}",total);
}
