use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn count_paints(map: &Vec<Vec<char>>, start_row: usize, start_col: usize, pattern: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 0..8 {
        for j in 0..8 {
            if map[start_row + i][start_col + j] != pattern[i][j] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let row_col: Vec<usize> = input_user_to_vec();
    let (row, col): (usize, usize) = (row_col[0], row_col[1]);

    let mut map: Vec<Vec<char>> = Vec::with_capacity(row);
    for _ in 0..row {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        let row_chars: Vec<char> = input_string.trim().chars().collect();
        map.push(row_chars);
    }

    let pattern_b: Vec<Vec<char>> = (0..8)
        .map(|i| (0..8).map(|j| if (i + j) % 2 == 0 { 'B' } else { 'W' }).collect())
        .collect();
    let pattern_w: Vec<Vec<char>> = (0..8)
        .map(|i| (0..8).map(|j| if (i + j) % 2 == 0 { 'W' } else { 'B' }).collect())
        .collect();

    let mut min_paints = usize::MAX;

    for i in 0..=(row - 8) {
        for j in 0..=(col - 8) {
            let paints_b = count_paints(&map, i, j, &pattern_b);
            let paints_w = count_paints(&map, i, j, &pattern_w);
            min_paints = min_paints.min(paints_b).min(paints_w);
        }
    }

    println!("{}", min_paints);
}
