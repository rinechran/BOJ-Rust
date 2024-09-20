use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let input: Vec<f64> = input_user_to_vec();
    let n = input[0];
    let l = input[1] as i32;

    for length in l..=100 {
        let length_f64 = length as f64;

        let a = (2.0 * n - (length_f64 * length_f64) + length_f64) / (2.0 * length_f64);

        if a.fract() == 0.0 && a >= 0.0 {
            let start = a as i32;

            let result: Vec<String> = (start..start + length).map(|x| x.to_string()).collect();
            println!("{}", result.join(" "));
            return;
        }
    }

    println!("-1");
}
