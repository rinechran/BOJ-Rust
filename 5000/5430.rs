use std::collections::VecDeque;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn parse_bracketed_array() -> Vec<i32> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .trim_matches(&['[', ']'][..])
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn input_user_to_string_to_vec_chars() -> Vec<char> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string.trim().chars().collect::<Vec<char>>()
}

fn main() {
    let test_cases: usize = input_user_to_vec()[0];

    for _ in 0..test_cases {
        let commands = input_user_to_string_to_vec_chars();
        let _: usize = input_user_to_vec()[0];
        let array: Vec<i32> = parse_bracketed_array();
        let mut deque: VecDeque<i32> = array.into_iter().collect();

        let mut reverse = false;
        let mut error_flag = false;

        for command in commands {
            match command {
                'R' => reverse = !reverse,
                'D' => {
                    if reverse {
                        if deque.pop_back().is_none() {
                            println!("error");
                            error_flag = true;
                            break;
                        }
                    } else {
                        if deque.pop_front().is_none() {
                            println!("error");
                            error_flag = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        if error_flag {
            continue;
        }

        let result: Vec<i32> = if reverse {
            deque.into_iter().rev().collect()
        } else {
            deque.into_iter().collect()
        };

        println!(
            "[{}]",
            result
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
