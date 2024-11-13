use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_cut_len(cables: &Vec<usize>, want_cable: usize) -> usize {
    let max_cable = match cables.iter().max() {
        Some(&max) => max,
        None => return 0,
    };

    let mut start = 1;
    let mut end = max_cable;
    let mut result = 0;

    while start <= end {
        let mid = (start + end) / 2;
        let total_cuts: usize = cables.iter().map(|&cable| cable / mid).sum();

        if total_cuts >= want_cable {
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
    let cable_count = row[0];
    let want_cable_count = row[1];

    let mut cables: Vec<usize> = Vec::new();

    for _ in 0..cable_count {
        let cable_length: Vec<usize> = input_user_to_vec();
        cables.push(cable_length[0]);
    }

    let result = get_cut_len(&cables, want_cable_count);
    println!("{}", result);
}
