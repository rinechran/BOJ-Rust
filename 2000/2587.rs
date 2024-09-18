use std::io;
use std::io::{Read, stdin};
use std::str::FromStr;

fn input_user_to_vec<T : FromStr > () -> Vec<T>{
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn main() {
    let m = 0;

    let mut vector : Vec<i32> = Vec::new();

    for _ in 0..5{
        let num = input_user_to_vec()[0];
        vector.push(num);
    }
    vector.sort();
    let total: i32 = vector.iter().sum::<i32>();
    println!("{}",total /5) ;
    println!("{}",vector[2]);

}
