use std::io::stdin;

fn main() {

    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    let mut numbers : Vec<i32> = input_string
        .trim()
        .chars()
        .map(|c | c.to_digit(10).unwrap() as i32)
        .collect();


    numbers.sort_by(|a,b| b.cmp(a));

    for i in numbers{
        print!("{}",i);
    }
}
