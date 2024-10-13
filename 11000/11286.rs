use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr, R: BufRead>(reader: &mut R) -> Vec<T> {
    let mut input_string = String::new();
    reader.read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let input: Vec<i32> = input_user_to_vec(&mut reader);
    let n = input[0];
    let mut min_heap = BinaryHeap::new();

    for _ in 0..n {
        let input: Vec<i32> = input_user_to_vec(&mut reader);
        let x = input[0];
        if x != 0 {
            min_heap.push(Reverse((x.abs(), x)));
        } else {
            if let Some(Reverse((_, value))) = min_heap.pop() {
                writeln!(writer, "{}", value).unwrap();
            } else {
                writeln!(writer, "{}", 0).unwrap();
            }
        }
    }

    writer.flush().unwrap();
}
