use std::collections::VecDeque;
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
    let row: Vec<usize> = input_user_to_vec();
    let map_size = row[0];
    let row: Vec<usize> = input_user_to_vec();
    let edge_count = row[0];

    let mut map = vec![vec![0usize; map_size + 1]; map_size + 1];
    let mut is_visit = vec![false; map_size + 1];

    for _ in 0..edge_count {
        let edge: Vec<usize> = input_user_to_vec();
        let start = edge[0];
        let end = edge[1];
        map[start][end] = 1;
        map[end][start] = 1;
    }

    let mut queue = VecDeque::new();
    queue.push_back(1);
    is_visit[1] = true;

    let mut count = 0;

    while let Some(current) = queue.pop_front() {
        count += 1;
        for i in 1..=map_size {
            if !is_visit[i] && map[current][i] == 1 {
                queue.push_back(i);
                is_visit[i] = true;
            }
        }
    }

    println!("{}", count-1);
}
