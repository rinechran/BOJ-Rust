use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let stdin = io::stdin();
    let mut input_string = String::new();
    stdin.read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let row: Vec<i32> = input_user_to_vec();
    let n = row[0] as usize;
    let m = row[1] as usize;

    let mut name_by_number: HashMap<String, i32> = HashMap::new();
    let mut number_by_name: HashMap<i32, String> = HashMap::new();

    for i in 1..=n {
        let row: Vec<String> = input_user_to_vec();
        let name = row[0].clone();
        name_by_number.insert(name.clone(), i as i32);
        number_by_name.insert(i as i32, name);
    }

    for _ in 0..m {
        let query: Vec<String> = input_user_to_vec();
        let query_str = &query[0];

        if let Ok(num) = query_str.parse::<i32>() {
            if let Some(name) = number_by_name.get(&num) {
                writeln!(writer, "{}", name).unwrap();
            }
        } else {
            if let Some(number) = name_by_number.get(query_str) {
                writeln!(writer, "{}", number).unwrap();
            }
        }
    }
}

