use std::cmp::max;
use std::collections::BinaryHeap;
use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let input: Vec<usize> = input_user_to_vec();
    let nc = input[0];
    let mut max_heap = BinaryHeap::new();

    for _ in 0..nc{
        let input: Vec<usize> = input_user_to_vec();
        let input = input[0];
        if input != 0 {
            max_heap.push(input);
            continue;
        }

        if let Some(value) = max_heap.pop(){
            println!("{}",value);
        }
        else{
            println!("{}",0);
        }
    }




}
