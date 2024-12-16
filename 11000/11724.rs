use std::collections::{HashMap, VecDeque};
use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn solve(n: usize, k: usize) -> usize {
    let mut graph = HashMap::new();

    for _ in 0..k {
        let input: Vec<usize> = input_user_to_vec();
        graph.entry(input[0]).or_insert_with(Vec::new).push(input[1]);
        graph.entry(input[1]).or_insert_with(Vec::new).push(input[0]);
    }

    let mut is_visited = vec![false; n + 1];
    let mut result = 0;

    for i in 1..=n {
        if !is_visited[i] {
            result += 1;
            let mut queue = VecDeque::new();
            queue.push_back(i);

            while let Some(node) = queue.pop_front() {
                if is_visited[node] {
                    continue;
                }
                is_visited[node] = true;

                if let Some(neighbors) = graph.get(&node) {
                    for &neighbor in neighbors {
                        if !is_visited[neighbor] {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let input: Vec<usize> = input_user_to_vec();
    let n = input[0];
    let k = input[1];
    let result = solve(n, k);
    println!("{}", result);
}
