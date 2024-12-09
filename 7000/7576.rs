use std::collections::{HashMap, VecDeque};
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn init_map(n : isize) -> Vec<Vec<isize>>{
    let mut map : Vec<Vec<isize>> = Vec::new();
    for _ in 0..n{
        let row : Vec<isize> = input_user_to_vec();
        map.push(row);
    }
    map
}

fn find_tomato(map: &Vec<Vec<isize>>) -> Vec<(usize, usize)> {
    let mut ripe_tomatoes = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 1 {
                ripe_tomatoes.push((i, j));
            }
        }
    }
    ripe_tomatoes
}
fn simulation(map : &Vec<Vec<isize>>) -> i32{
    let mut map = map.clone();
    let mut queue = VecDeque::new();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rows = map.len();
    let cols = map[0].len();

    for &(x, y) in find_tomato(&map).iter() {
        queue.push_back((x, y, 0));
    }

    let mut max_days = 0;

    while let Some((x, y, day)) = queue.pop_front() {
        max_days = max_days.max(day);
        for &(dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                let nx = nx as usize;
                let ny = ny as usize;
                if map[nx][ny] == 0 {
                    map[nx][ny] = 1;
                    queue.push_back((nx, ny, day + 1));
                }
            }
        }
    }

    if map.iter().any(|row| row.contains(&0)) {
        return -1;
    }

    max_days
}
fn main() {
    let mut row: Vec<isize> = input_user_to_vec();

    let n = row[1];
    let map = init_map(n);

    let result = simulation(&map);

    println!("{}",result);

}
