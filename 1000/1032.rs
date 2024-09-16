use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn input_user() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n: usize = input_user_to_vec()[0];
    let mut vec_str: Vec<String> = vec![];

    for _ in 0..n {
        let temp = input_user();
        vec_str.push(temp);
    }

    let first_str = &vec_str[0];
    let str_len = first_str.len();

    for i in 0..str_len {
        let temp_char = first_str.chars().nth(i).unwrap();
        let mut is_diff = false;

        for j in 1..n {
            if vec_str[j].chars().nth(i).unwrap() != temp_char {
                is_diff = true;
                break;
            }
        }

        if is_diff {
            print!("?");
        } else {
            print!("{}", temp_char);
        }
    }
    println!();  // Newline after output
}
