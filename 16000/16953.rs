use std::collections::{HashSet, VecDeque};
use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn solve(a: isize, want: isize) -> isize {
    if a == want {
        return 0;
    }

    let mut queue = VecDeque::new();

    queue.push_back((a, 1)); 

    while let Some((data, time)) = queue.pop_front() {
        if data == want {
            return time;
        }

        let next1 = data * 2;
        if next1 <= want {
            queue.push_back((next1, time + 1));
        }
        let next2 = data * 10 + 1;
        if next2 <= want {
            queue.push_back((next2, time + 1));
        }
    }

    -1
}

fn main() {
    let row: Vec<isize> = input_user_to_vec();
    let a = row[0];
    let b = row[1];

    println!("{}", solve(a, b));
}
