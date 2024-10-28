use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let expression = input.trim();

    let parts: Vec<&str> = expression.split('-').collect();
    let mut result = 0;

    let first_part = parts[0];
    result += first_part.split('+')
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>();

    for part in parts.iter().skip(1) {
        let sum: i32 = part.split('+')
            .map(|x| x.parse::<i32>().unwrap())
            .sum();
        result -= sum;
    }

    println!("{}", result);
}
