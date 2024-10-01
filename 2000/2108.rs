use std::collections::HashMap;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn calculate_mean(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    (sum as f64 / numbers.len() as f64).round() as i32
}

fn calculate_median(numbers: &Vec<i32>) -> i32 {
    numbers[numbers.len() / 2]
}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    numbers.iter().for_each(|&num| {
        *frequency_map.entry(num).or_insert(0) += 1;
    });

    let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);
    let mut most_frequent_elements: Vec<i32> = frequency_map
        .into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .map(|(key, _)| key)
        .collect();

    most_frequent_elements.sort();

    if most_frequent_elements.len() > 1 {
        most_frequent_elements[1]
    } else {
        most_frequent_elements[0]
    }
}

fn calculate_range(numbers: &Vec<i32>) -> i32 {
    numbers[numbers.len() - 1] - numbers[0]
}

fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];

    let mut numbers: Vec<i32> = (0..n).map(|_| input_user_to_vec()[0]).collect();

    numbers.sort();

    println!("{}", calculate_mean(&numbers));
    println!("{}", calculate_median(&numbers));
    println!("{}", calculate_mode(&numbers));
    println!("{}", calculate_range(&numbers));
}
