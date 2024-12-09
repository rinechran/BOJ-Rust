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

fn bfs(map: &HashMap<usize, Vec<usize>>, start: usize) {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut is_visit = HashMap::new();
    let mut visit_order = Vec::new();

    while let Some(index) = queue.pop_front() {
        if *is_visit.get(&index).unwrap_or(&false) {
            continue;
        }

        visit_order.push(index);
        is_visit.insert(index, true);

        if let Some(neighbors) = map.get(&index) {
            for &neighbor in neighbors {
                if !*is_visit.get(&neighbor).unwrap_or(&false) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    let result = visit_order
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);
}

fn dfs_recursive(
    map: &HashMap<usize, Vec<usize>>,
    current: usize,
    is_visit: &mut HashMap<usize, bool>,
    visit_order: &mut Vec<usize>,
) {

    is_visit.insert(current, true);
    visit_order.push(current);



    if let Some(neighbors) = map.get(&current) {
        for &neighbor in neighbors {
            if !*is_visit.get(&neighbor).unwrap_or(&false) {
                dfs_recursive(map, neighbor, is_visit, visit_order);
            }
        }
    }
}
fn dfs(map: &HashMap<usize,Vec<usize>> , start : usize) {

    let mut is_visit = HashMap::new();
    let mut visit_order = Vec::new();

    dfs_recursive(map, start, &mut is_visit, &mut visit_order);

    let result = visit_order
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);

}

fn init_map(m : usize) -> HashMap<usize,Vec<usize>>{

    let mut map = HashMap::new();
    for _ in 0..m{
        let row : Vec<usize> = input_user_to_vec();
        let v = row[0];
        let u = row[1];
        map.entry(v).or_insert(Vec::new()).push(u);
        map.entry(u).or_insert(Vec::new()).push(v);
    }
    for arr in map.values_mut() {
        arr.sort();
    }
    map
}
fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let m = row[1];
    let v = row[2];
    let map = init_map(m);
    dfs(&map,v);
    bfs(&map,v);

}
