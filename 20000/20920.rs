use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let parts: Vec<usize> = input_user_to_vec();
    let n = parts[0];
    let m = parts[1];

    let mut word_count: HashMap<String, usize> = HashMap::new();

    for _ in 0..n {
        let line: Vec<String> = input_user_to_vec();
        let word = line[0].clone();
        if word.len() >= m {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    let mut words: Vec<(String, usize)> = word_count.into_iter().collect();

    words.sort_by(|a, b| {
        if b.1 != a.1 {
            return b.1.cmp(&a.1);
        }
        if b.0.len() != a.0.len() {
            return b.0.len().cmp(&a.0.len());
        }
        return a.0.cmp(&b.0);
    });

    for (word, _) in words {
        writeln!(writer, "{}", word).unwrap();
    }
}
