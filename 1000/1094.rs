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
    let input: Vec<usize> = input_user_to_vec();
    let want = input[0];

    let mut count = 0;
    let mut len = 64;
    let mut current_len = 0;

    if want==64{
        println!("{}", 1);
    }
    else{
        while current_len < want {
            len /= 2;
            if current_len + len <= want {
                current_len += len;
                count += 1;
            }
        }
        println!("{}", count);
    }

}
