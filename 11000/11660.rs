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
    let n = input[0];
    let m = input[1];

    let mut table = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        let row: Vec<i32> = input_user_to_vec(&mut reader);
        for j in 1..=n {
            table[i][j] = row[j - 1];
        }
    }

    let mut prefix_sum = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            prefix_sum[i][j] = table[i][j]
                + prefix_sum[i - 1][j]
                + prefix_sum[i][j - 1]
                - prefix_sum[i - 1][j - 1];
        }
    }

    for _ in 0..m {
        let query: Vec<usize> = input_user_to_vec(&mut reader);
        let x1 = query[0];
        let y1 = query[1];
        let x2 = query[2];
        let y2 = query[3];

        let result = prefix_sum[x2][y2]
            - prefix_sum[x1 - 1][y2]
            - prefix_sum[x2][y1 - 1]
            + prefix_sum[x1 - 1][y1 - 1];

        writeln!(writer, "{}", result).unwrap();
    }
}
