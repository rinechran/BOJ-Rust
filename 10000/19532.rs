use std::io;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let input: Vec<isize> = input_user_to_vec();
    let (a1, b1, c1) = (input[0], input[1], input[2]);
    let (a2, b2, c2) = (input[3], input[4], input[5]);

    for x in -999..=999 {
        for y in -999..=999 {
            if a1 * x + b1 * y == c1 && a2 * x + b2 * y == c2 {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}
