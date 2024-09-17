use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number : i32 = input.trim().parse().unwrap();

    let x = 2541 - 1998;
    print!("{}",number-x);

}