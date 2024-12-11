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
    let m = input[1];

    let array: Vec<isize> = input_user_to_vec();

    let mut array_deque: VecDeque<isize> = (1..=n).collect();

    let mut total_moves = 0;

    for target in array {
        let target_idx = array_deque
            .iter()
            .position(|&x| x == target)
            .unwrap();

        let left_moves = target_idx;
        let right_moves = array_deque.len() - target_idx;

        total_moves += left_moves.min(right_moves);

        if left_moves <= right_moves {
            for _ in 0..left_moves {
                let front = array_deque.pop_front().unwrap();
                array_deque.push_back(front);
            }
        } else {
            for _ in 0..right_moves {
                let back = array_deque.pop_back().unwrap();
                array_deque.push_front(back);
            }
        }
        array_deque.pop_front();
    }

    println!("{}", total_moves);
}
