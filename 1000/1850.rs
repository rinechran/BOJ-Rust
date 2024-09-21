use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn main() {
    let input: Vec<u64> = input_user_to_vec();
    let n = input[0];
    let l = input[1];

    for _ in 0..gcd(n,l){
        print!("{}",1);
    }
}
