use std::collections::{HashMap, VecDeque};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let input: Vec<isize> = input_user_to_vec();
    let n = input[0];
    let input: Vec<isize> = input_user_to_vec();
    let m = input[0];

    let n_str = n.to_string();
    let prefix = &n_str[..n_str.len() - 2];
    let mut min_val = isize::MAX;
    let mut min_val_string= String::new();

    for i in 0..=9 {
        for j in 0..=9 {
            let suffix = format!("{}{}", i, j);
            let suffix_num : isize = suffix.parse().ok().unwrap();

            let candidate_str = format!("{}{}", prefix, suffix);
            if let Ok(candidate) = candidate_str.parse::<isize>() {
                if candidate % m == 0 && suffix_num < min_val {
                    min_val = min_val.min(suffix_num);
                    min_val_string = suffix.to_string();
                }
            }
        }
    }

    println!("{}", min_val_string);
}
