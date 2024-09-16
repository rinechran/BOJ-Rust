use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn fast_pow(mut a: i64, mut b: i64, mod_val: i64) -> i64 {
    let mut result = 1;
    a = a % mod_val; 

    while b > 0 {
        if b % 2 == 1 {
            result = (result * a) % mod_val;
        }
        a = (a * a) % mod_val;
        b /= 2;
    }
    result
}
fn fast_pow_recursive(a: i64, n: i64, mod_val: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let half_pow = fast_pow_recursive(a, n / 2, mod_val);

    let half_pow_mod = (half_pow * half_pow) % mod_val;

    return if n % 2 == 1 {
        (half_pow_mod * a) % mod_val
    } else {
        half_pow_mod
    }
}
fn main() {
    let n = input_user_to_vec()[0];
    for _ in 0..n {
        let input = input_user_to_vec();
        let mut pow_result = fast_pow_recursive(input[0], input[1], 10);
        if pow_result == 0 {
            pow_result = 10;
        }
        println!("{}", pow_result);
    }
}