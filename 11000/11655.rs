use std::io::{self, stdin};

fn input_user_to_string() -> String {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string.trim().to_string()
}

fn main() {

    let input = input_user_to_string();

    let transform_input : String = input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + 13) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + 13) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect();

    println!("{}", transform_input);
}
