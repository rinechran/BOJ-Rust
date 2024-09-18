use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr>(reader: &mut BufReader<io::StdinLock>) -> Vec<T> {
    let mut input_string = String::new();
    reader.read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let case: i32 = input_user_to_vec(&mut reader)[0];

    let mut positions: Vec<(i32, i32)> = Vec::new();

    for _ in 0..case {
        let input: Vec<i32> = input_user_to_vec(&mut reader);
        positions.push((input[0], input[1]));
    }

    positions.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    for (x, y) in positions {
        writeln!(writer, "{} {}", x, y).unwrap();
    }

    writer.flush().unwrap();
}
