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

    let row: Vec<usize> = input_user_to_vec();

    let mut deque : VecDeque<usize>= (1..=row[0])
        .collect();
    let k = row[1];

    let mut result_arr : Vec<usize> = Vec::new();
    let mut index = 0;
    while !deque.is_empty() {
        index = (index + k - 1) % deque.len();
        result_arr.push(deque.remove(index).unwrap());
    }

    print!("<");
    let string_representation: String = result_arr
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    print!("{}", string_representation);
    print!(">");

}
