use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;
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

    let input: Vec<usize> = input_user_to_vec(&mut reader);
    let nc = input[0];
    let mut max_heap = BinaryHeap::new();

    for _ in 0..nc {
        let input: Vec<usize> = input_user_to_vec(&mut reader);
        let input = input[0];
        if input != 0 {
            max_heap.push(Reverse(input));
            continue;
        }

        if let Some(Reverse(value)) = max_heap.pop() {
            writeln!(writer, "{}", value).unwrap();
        } else {
            writeln!(writer, "{}", 0).unwrap();
        }
    }

    writer.flush().unwrap();
}
