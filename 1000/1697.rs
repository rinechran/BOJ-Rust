use std::collections::VecDeque;
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
    if n >= k {
        return n - k;
    }

    let max_limit = 100001;
    let mut visited = vec![false; max_limit];
    let mut queue = VecDeque::new();
    queue.push_back((n, 0));

    while let Some((current, time)) = queue.pop_front() {
        if current == k {
            return time;
        }

        if visited[current] {
            continue;
        }
        visited[current] = true;

        if current >= 1 && !visited[current - 1] {
            queue.push_back((current - 1, time + 1));
        }
        if current + 1 < max_limit && !visited[current + 1] {
            queue.push_back((current + 1, time + 1));
        }
        if current * 2 < max_limit && !visited[current * 2] {
            queue.push_back((current * 2, time + 1));
        }
    }

    unreachable!();
}

fn main() {
    let input: Vec<usize> = input_user_to_vec();
    let n = input[0];
    let k = input[1];
    let result = solve(n, k);
    println!("{}", result);
}
