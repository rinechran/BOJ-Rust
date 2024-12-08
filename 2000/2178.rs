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

fn bfs(map: &mut Vec<Vec<i32>>, n: usize, m: usize) -> i32 {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = VecDeque::new();

    queue.push_back((1, 1));
    map[1][1] = 1;

    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in directions.iter() {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            let ny = ny as usize;
            let nx = nx as usize;

            if map[ny][nx] == 1 {
                map[ny][nx] = map[y][x] + 1;
                queue.push_back((ny, nx));
            }
        }
    }

    map[n][m]
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let n = row[0];
    let m = row[1];

    let mut map: Vec<Vec<i32>> = vec![vec![0; m + 2]; n + 2];

    for j in 1..=n {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        let vec: Vec<i32> = input_string
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();
        for ch in 0..vec.len() {
            map[j][ch + 1] = vec[ch];
        }
    }

    let result = bfs(&mut map, n, m);

    println!("{}", result);
}
