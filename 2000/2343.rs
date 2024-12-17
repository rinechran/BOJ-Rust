use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_size_divide_into_groups(array: &Vec<isize>, max_size: isize) -> isize {
    let mut count = 1;
    let mut current_sum = 0;

    for &length in array {
        if current_sum + length > max_size {
            count += 1;
            current_sum = 0;
        }
        current_sum += length;
    }

    count
}

fn solve(array: Vec<isize>, group_count: isize) -> isize {
    let mut left = *array.iter().max().unwrap();
    let mut right: isize = array.iter().sum();
    let mut result = right;

    while left <= right {
        let mid = (left + right) / 2;
        let divide_size = get_size_divide_into_groups(&array, mid);

        if divide_size <= group_count {
            result = result.min(mid);
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    result
}

fn main() {
    let row: Vec<isize> = input_user_to_vec();
    let n = row[0];
    let m = row[1];

    let lectures: Vec<isize> = input_user_to_vec();
    let result = solve(lectures, m);
    println!("{}", result);
}
