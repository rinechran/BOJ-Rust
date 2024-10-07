use std::cmp;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn fast_max_sum(a: &Vec<i32>) -> i32 {
    if a.is_empty() {
        return 0;
    }

    let mut sum = 0;
    let mut ret = -1000000;
    for &val in a.iter() {
        sum = cmp::max(sum, 0) + val;
        ret = cmp::max(ret, sum);
    }
    ret
}

fn main() {

    let input : Vec<usize> = input_user_to_vec();
    let input : Vec<i32> = input_user_to_vec();
    println!("{}",fast_max_sum(&input));
}
