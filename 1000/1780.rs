use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn compress(map: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> (i32, i32, i32) {
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
        match current_key {
            -1 => return (1, 0, 0),
            0 => return (0, 1, 0),
            1 => return (0, 0, 1),
            _ => unreachable!(),
        }
    }

    let new_size = size / 3;
    let mut count = (0, 0, 0);
    for i in 0..3 {
        for j in 0..3 {
            let sub_count = compress(map, x + i * new_size, y + j * new_size, new_size);
            count.0 += sub_count.0;
            count.1 += sub_count.1;
            count.2 += sub_count.2;
        }
    }

    count
}

fn main() {
    let n: usize = {
        let row: Vec<i32> = input_user_to_vec();
        row[0] as usize
    };

    let mut map: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let row: Vec<i32> = input_user_to_vec();
        map.push(row);
    }

    let result = compress(&map, 0, 0, n);
    println!("{}", result.0);
    println!("{}", result.1);
    println!("{}", result.2);
}
