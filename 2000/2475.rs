use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {
    let row: Vec<i64> = input_user_to_vec();

    let total : i64 = row
        .iter()
        .map(|s| s*s)
        .sum();

    let result = total % 10 ;
    println!("{}",result);

}
