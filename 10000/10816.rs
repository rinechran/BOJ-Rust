use std::collections::{BTreeSet, HashMap};
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
    let san: Vec<i32> = input_user_to_vec();

    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for &number in &san {
        let counter = count_map.entry(number).or_insert(0);
        *counter += 1;
    }

    let temp: BTreeSet<i32> = input_user_to_vec();
    let cards: Vec<i32> = input_user_to_vec();

    for i in cards {
        let count = count_map.get(&i).unwrap_or(&0);
        print!("{} ", count);
    }
}
