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

fn generate_sequence(limit: usize) -> Vec<usize> {
    let mut sequence = Vec::new();
    let mut num = 1;
    while sequence.len() < limit {
        sequence.extend(vec![num; num]);
        num += 1;
    }
    sequence
}
fn main() {

    let nums : Vec<usize> = input_user_to_vec();
    let (a, b) = (nums[0], nums[1]);

    let sequence = generate_sequence(b);
    let result: usize = sequence[a-1..b].iter().sum();
    println!("{}", result);


}
