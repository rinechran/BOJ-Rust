use std::collections::{HashMap, VecDeque};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn input_user_string() -> String {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .to_string()

}

fn main() {

    let table: HashMap<&str, (usize, usize)> = [
        ("black", (0, 1)),
        ("brown", (1, 10)),
        ("red", (2, 100)),
        ("orange", (3, 1_000)),
        ("yellow", (4, 10_000)),
        ("green", (5, 100_000)),
        ("blue", (6, 1_000_000)),
        ("violet", (7, 10_000_000)),
        ("grey", (8, 100_000_000)),
        ("white", (9, 1_000_000_000)),
    ]
        .iter()
        .cloned()
        .collect();

    let mut result =0;
    for i in 0..3{
        let input = input_user_string();
        let find = table.get(input.as_str()).unwrap();

        if i==2{
            result*=find.1;
            break;
        }
        result = (result * 10) +find.0;
    }
    println!("{}",result);

}
