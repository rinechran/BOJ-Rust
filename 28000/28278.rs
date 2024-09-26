use std::io::{self, BufRead, BufWriter, Write};

fn input_user_to_vec<T: std::str::FromStr>(input: &mut impl BufRead) -> Vec<T> {
    let mut input_string = String::new();
    input.read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let n: usize = input_user_to_vec::<usize>(&mut reader)[0];
    let mut stack: Vec<i32> = Vec::new();

    for _ in 0..n {
        let row: Vec<i32> = input_user_to_vec(&mut reader);

        let command = row[0];

        match command {
            1 => {
                let x = row[1];
                stack.push(x);
            }
            2 => {
                if let Some(top) = stack.pop() {
                    writeln!(writer, "{}", top).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            3 => {
                writeln!(writer, "{}", stack.len()).unwrap();
            }
            4 => {
                writeln!(writer, "{}", if stack.is_empty() { 1 } else { 0 }).unwrap();
            }
            5 => {
                if let Some(&top) = stack.last() {
                    writeln!(writer, "{}", top).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            _ => {}
        }
    }
}
