use std::collections::VecDeque;
use std::io::{self, BufRead, Write, BufWriter};
use std::str::FromStr;

fn input_user_to_vec<T: FromStr>() -> Vec<T> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let row: Vec<i32> = input_user_to_vec();
    let tc = row[0];

    let mut queue: VecDeque<i32> = VecDeque::new();

    for _ in 0..tc {
        let input: Vec<String> = input_user_to_vec();
        match input[0].as_str() {
            "push" => {
                let num: i32 = input[1].parse().unwrap();
                queue.push_back(num);
            },
            "pop" => {
                if let Some(pop_data) = queue.pop_front() {
                    writeln!(writer, "{}", pop_data).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            "size" => {
                writeln!(writer, "{}", queue.len()).unwrap();
            },
            "empty" => {
                writeln!(writer, "{}", if queue.is_empty() { 1 } else { 0 }).unwrap();
            },
            "front" => {
                if let Some(front_data) = queue.front() {
                    writeln!(writer, "{}", front_data).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            "back" => {
                if let Some(back_data) = queue.back() {
                    writeln!(writer, "{}", back_data).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            _ => {}
        }
    }

    writer.flush().unwrap();
}
