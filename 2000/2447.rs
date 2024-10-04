use std::io::{self, BufRead, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_root(val: usize) -> u32 {
    for i in 1..=8 {
        if 3usize.pow(i) == val {
            return i as u32;
        }
    }
    0
}

fn show_map(map: &Vec<Vec<char>>) {
    for r in 0..map.len() {
        for i in 0..map[r].len() {
            print!("{}", map[r][i]);
        }
        println!();
    }
}

fn make_map(map: &mut Vec<Vec<char>>, x: usize, y: usize, size: usize) {
    if size == 1 {
        return;
    }

    let len = size / 3;

    for i in x + len..x + 2 * len {
        for j in y + len..y + 2 * len {
            map[i][j] = ' ';
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            make_map(map, x + i * len, y + j * len, len);
        }
    }
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let n = row[0];
    let k = get_root(n);

    let mut map: Vec<Vec<char>> = vec![vec!['*'; n]; n];
    make_map(&mut map, 0, 0, n);
    show_map(&map);
}
