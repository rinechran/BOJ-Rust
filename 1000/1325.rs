use std::collections::{VecDeque, HashMap};
use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn initialize_graph(edge_count: usize) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    for _ in 0..edge_count {
        let edge: Vec<usize> = input_user_to_vec();
        let start = edge[0];
        let end = edge[1];

        graph.entry(end).or_insert_with(Vec::new).push(start);
    }
    graph
}

fn bfs(graph: &HashMap<usize, Vec<usize>>, start_node: usize, map_size: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start_node);

    let mut visited = vec![false; map_size + 1];
    visited[start_node] = true;

    let mut count = 0;
    while let Some(current) = queue.pop_front() {
        count += 1;
        if let Some(neighbors) = graph.get(&current) {
            for &next_node in neighbors {
                if !visited[next_node] {
                    queue.push_back(next_node);
                    visited[next_node] = true;
                }
            }
        }
    }
    count
}

fn calculate_and_sort_results(graph: &HashMap<usize, Vec<usize>>, map_size: usize) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    for node in 1..=map_size {
        let reachable_count = bfs(graph, node, map_size);
        results.push((reachable_count, node));
    }
    results.sort_by(|a, b| b.0.cmp(&a.0));
    results
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let map_size = row[0];
    let edge_count = row[1];

    let graph = initialize_graph(map_size, edge_count);
    let results = calculate_and_sort_results(&graph, map_size);

    let max_val = results[0].0;
    for (count, node) in results {
        if count != max_val {
            break;
        }
        print!("{} ", node);
    }
}
