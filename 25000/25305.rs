use std::io::{Read, stdin};

fn input_user_to_vec<T : std::str::FromStr > () -> Vec<T>{
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn main() {
    let row: Vec<usize> = input_user_to_vec();

    let k = row[1];

    let mut vector : Vec<i32> = input_user_to_vec();


    vector.sort_by( |a,b| b.cmp(a));

    println!("{}",vector[k-1]);

}
