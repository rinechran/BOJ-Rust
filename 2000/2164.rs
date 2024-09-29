use std::collections::VecDeque;
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

    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];
    let mut queue: VecDeque<i32> = (1..=n).collect();

    while queue.len() != 1 {
        queue.pop_front();
        let q = queue.pop_front().unwrap();
        queue.push_back(q);
    }

    println!("{}",queue[0]);

}
