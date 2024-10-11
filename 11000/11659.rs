use std::io::{self, BufRead, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr>(reader: &mut dyn BufRead) -> Vec<T> {
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
    let row_count = input[0];
    let nc = input[1];

    let row: Vec<i32> = input_user_to_vec(&mut reader);
    let mut dp = vec![0; row_count + 1];

    for i in 1..=row_count {
        dp[i] = dp[i - 1] + row[i - 1];
    }

    for _ in 0..nc {
        let query: Vec<usize> = input_user_to_vec(&mut reader);
        let left = query[0];
        let right = query[1];
        let result = dp[right] - dp[left - 1];
        writeln!(writer, "{}", result).unwrap();
    }
}
