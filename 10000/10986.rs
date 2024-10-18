use std::io::{self, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn input_user_to_vec<T: std::str::FromStr>(reader: &mut dyn BufRead) -> Vec<T> {
    let mut input_string = String::new();
    reader.read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn nC2(n: i64) -> i64 {
    if n < 2 {
        0
    } else {
        (n * (n - 1)) / 2
    }
}

fn calculate_modular_sums(row: &[i64], m: i64) -> Vec<i64> {
    let mut prefix_sum = 0;
    let mut modular = vec![0; row.len()];

    for i in 0..row.len() {
        prefix_sum += row[i];
        modular[i] = prefix_sum % m;
    }
    modular
}

fn count_remainders(modular: &[i64]) -> HashMap<i64, i64> {
    let mut remainder_count = HashMap::new();

    for &mod_value in modular {
        *remainder_count.entry(mod_value).or_insert(0) += 1;
    }

    remainder_count
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let input: Vec<i64> = input_user_to_vec(&mut reader);
    let n = input[0];
    let m = input[1];

    let row: Vec<i64> = input_user_to_vec(&mut reader);

    let modular = calculate_modular_sums(&row, m);

    let remainder_count = count_remainders(&modular);

    let mut total = 0;

    for &count in remainder_count.values() {
        total += nC2(count);
    }

    for &mod_value in &modular {
        if mod_value == 0 {
            total += 1;
        }
    }
    writeln!(writer, "{}", total).unwrap();
}
