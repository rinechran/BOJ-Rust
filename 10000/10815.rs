use std::collections::BTreeSet;
use std::io::stdin;
use std::iter::FromIterator;

fn input_user_to_vec<T: std::str::FromStr, K: FromIterator<T>>() -> K {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let temp: BTreeSet<i32> = input_user_to_vec();
    let san: BTreeSet<i32> = input_user_to_vec();
    let temp: BTreeSet<i32> = input_user_to_vec();
    let cards: Vec<i32> = input_user_to_vec();

    for &i in &cards {
        if san.contains(&i) {
            print!("{} ", 1);
        } else {
            print!("0 ");
        }
    }
}
