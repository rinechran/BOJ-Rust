use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_tree_len(trees: &Vec<usize>, want_len: usize) -> usize {
    let max_tree_len = match trees.iter().max() {
        Some(&max) => max,
        None => return 0,
    };
    let mut start = 0;
    let mut end = max_tree_len;
    let mut result = 0;

    while start <= end {
        let mid = (start + end) / 2;
        let total_sum: usize = trees
            .iter()
            .filter(|&&tree| tree > mid)
            .map(|&tree| tree - mid)
            .sum();

        if total_sum >= want_len {
            result = mid;
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    result
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let tree_len = row[1];

    let trees: Vec<usize> = input_user_to_vec();

    let result = get_tree_len(&trees, tree_len);
    println!("{}", result);
}
