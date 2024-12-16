use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn count_subsequences(array: &Vec<isize>, index: usize, current_sum: isize, target: isize) -> usize {
    if index == array.len() {
        if current_sum == target {
            return 1;
        } else {
            return 0;
        }
    }

    let exclude = count_subsequences(array, index + 1, current_sum, target);
    let include = count_subsequences(array, index + 1, current_sum + array[index], target);

    exclude + include
}

fn main() {
    let input: Vec<isize> = input_user_to_vec();
    let n = input[0] as usize;
    let s = input[1];
    let array: Vec<isize> = input_user_to_vec();

    let mut result = count_subsequences(&array, 0, 0, s);

    if s == 0 {
        result -= 1;
    }

    println!("{}", result);
}
