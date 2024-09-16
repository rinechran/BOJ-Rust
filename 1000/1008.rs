use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers : Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    print!("{}",numbers[0] / numbers[1])

}