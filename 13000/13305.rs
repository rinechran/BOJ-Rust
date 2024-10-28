use std::cmp::Reverse;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[derive(Debug)]
struct OilAndIndex {
    price: usize,
    index: usize,
    target_len: usize,
}

fn main() {
    let _: Vec<usize> = input_user_to_vec();
    let mut len: Vec<usize> = input_user_to_vec();
    let mut oil_price: Vec<usize> = input_user_to_vec();

    let mut total_cost = 0;
    let mut min_price = oil_price[0];

    for i in 0..len.len() {
        if oil_price[i] < min_price {
            min_price = oil_price[i];
        }
        total_cost += min_price * len[i];
    }
    println!("{}", total_cost);
}
