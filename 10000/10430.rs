use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number : Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let A = number[0];
    let B = number[1];
    let C = number[2];

    println!("{}", (A+B)%C);
    println!("{}", ((A%C) + (B%C))%C);
    println!("{}", (A*B)%C);
    println!("{}", ((A%C) * (B%C))%C);

}