use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn input_user_to_char() -> Vec<char> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string.trim().chars().collect()
}

fn dfs(
    map: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    is_visit: &mut Vec<Vec<bool>>,
) -> isize {
    if x < 0 || x >= map[0].len() as isize || y < 0 || y >= map.len() as isize {
        return 0;
    }

    let x = x as usize;
    let y = y as usize;

    if is_visit[y][x] || map[y][x] == '0' {
        return 0;
    }

    is_visit[y][x] = true;

    let mut size = 1;
    size += dfs(map, x as isize - 1, y as isize, is_visit);
    size += dfs(map, x as isize + 1, y as isize, is_visit);
    size += dfs(map, x as isize, y as isize - 1, is_visit);
    size += dfs(map, x as isize, y as isize + 1, is_visit);

    size
}

fn solve_map(map: &Vec<Vec<char>>) -> Vec<isize> {
    let mut is_visit = vec![vec![false; map[0].len()]; map.len()];
    let mut areas = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !is_visit[y][x] && map[y][x] == '1' {
                let area_size = dfs(map, x as isize, y as isize, &mut is_visit);
                if area_size > 0 {
                    areas.push(area_size);
                }
            }
        }
    }

    areas
}

fn main() {
    let input: Vec<usize> = input_user_to_vec();
    let n = input[0];

    let mut map = Vec::new();
    for _ in 0..n {
        let row = input_user_to_char();
        map.push(row);
    }

    let mut areas = solve_map(&map);
    areas.sort();
    println!("{}", areas.len());
    for area in areas {
        println!("{}", area);
    }
}
