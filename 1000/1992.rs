use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn input_str_vec() -> Vec<char> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();
    input_string.trim().chars().collect()
}

fn compress(map: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> String {
    let current_key = map[x][y];
    let mut is_uniform = true;

    for i in x..x + size {
        for j in y..y + size {
            if map[i][j] != current_key {
                is_uniform = false;
                break;
            }
        }
        if !is_uniform {
            break;
        }
    }

    if is_uniform {
        return current_key.to_string();
    }

    let half = size / 2;
    let top_left = compress(map, x, y, half);
    let top_right = compress(map, x, y + half, half);
    let bottom_left = compress(map, x + half, y, half);
    let bottom_right = compress(map, x + half, y + half, half);

    format!("({}{}{}{})", top_left, top_right, bottom_left, bottom_right)
}

fn solve(map: &Vec<Vec<i32>>) -> String {
    compress(map, 0, 0, map.len())
}

fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n: usize = row[0] as usize;
    let mut map: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        let row = input_str_vec();
        for j in 0..n {
            map[i][j] = row[j].to_digit(10).unwrap() as i32;
        }
    }

    let result = solve(&map);
    println!("{}", result);
}
