use std::collections::BTreeSet;
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

    let case: i32 = input_user_to_vec()[0];

    let mut btree: BTreeSet<String> = BTreeSet::new();

    for _ in 0..case {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        let trimmed = input_string.trim().to_string(); 
        btree.insert(trimmed);
    }

    let mut vec: Vec<String> = btree.into_iter().collect();

    vec.sort_by(|a, b| {
        if a.len() != b.len() {
            a.len().cmp(&b.len()) 
        } else {
            a.cmp(b)
        }
    });

    for s in vec {
        println!("{}", s);
    }
}
