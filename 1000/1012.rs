use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn map_explore(y: usize, x: usize, map: &Vec<Vec<bool>>, is_visit_map: &mut Vec<Vec<bool>>) {
    let mut vector: Vec<(usize, usize)> = Vec::new();
    vector.push((y, x));

    let move_x: [i32; 4] = [0, 0, -1, 1];
    let move_y: [i32; 4] = [-1, 1, 0, 0];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    while let Some((current_y, current_x)) = vector.pop() {
        let move_count = move_y.len();

        for i in 0..move_count {
            let next_y = current_y as i32 + move_y[i];
            let next_x = current_x as i32 + move_x[i];

            if next_y < 0 || next_x < 0 || next_y >= height || next_x >= width {
                continue;
            }

            let next_y = next_y as usize;
            let next_x = next_x as usize;

            if is_visit_map[next_y][next_x] || !map[next_y][next_x] {
                continue;
            }

            is_visit_map[next_y][next_x] = true;
            vector.push((next_y, next_x));
        }
    }
}

fn run() {
    let input = input_user_to_vec();
    let col = input[0];
    let row = input[1];
    let k = input[2];

    let mut map = vec![vec![false; col]; row];
    let mut is_visit_map = vec![vec![false; col]; row];

    for _ in 0..k {
        let input = input_user_to_vec();
        let sel_x = input[0];
        let sel_y = input[1];
        map[sel_y][sel_x] = true;
    }

    let mut total = 0;

    for r in 0..row {
        for c in 0..col {
            let current_val = map[r][c];
            let is_visit = is_visit_map[r][c];

            if is_visit {
                continue;
            }

            if current_val {
                map_explore(r, c, &map, &mut is_visit_map);
                total += 1;
            }
        }
    }

    println!("{}", total);
}

fn main() {
    let tc = input_user_to_vec()[0];

    for _ in 0..tc {
        run();
    }
}
