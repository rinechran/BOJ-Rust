use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn run(n: usize, m: usize) {
    let mut vec: Vec<bool> = vec![true; m + 1];

    if m >= 2 {
        vec[0] = false;
        vec[1] = false;
    }

    for i in 2..=(m as f64).sqrt() as usize {
        if vec[i] == true {
            for j in (i*i..=m).step_by(i) {
                vec[j] = false;
            }
        }
    }

    for i in n..=m {
        if vec[i] == true {
            println!("{}", i);
        }
    }
}

fn main() {
    let row = input_user_to_vec();
    run(row[0], row[1]);
}
