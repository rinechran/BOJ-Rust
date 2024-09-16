use std::cmp::min;
use std::io;

fn input_string() ->String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input
}

fn main() {
    let input = input_string();
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let x = numbers[0];
    let y = numbers[1];
    let w = numbers[2];
    let h = numbers[3];
    let mut result = i32::MAX;

    result = min(x,w-x);
    result = min(result,y);
    result = min(result,h-y);
    println!("{}",result);

}
