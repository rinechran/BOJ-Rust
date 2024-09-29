use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Write, stdin, stdout};

fn input_user_to_vec<T: std::str::FromStr>(input: &mut dyn BufRead) -> Vec<T> {
    let mut input_string = String::new();
    input.read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdin = stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let row: Vec<i32> = input_user_to_vec(&mut reader);
    let n = row[0] as usize;
    let mut deque: VecDeque<i32> = VecDeque::new();

    for _ in 0..n {
        let command: Vec<i32> = input_user_to_vec(&mut reader);
        match command[0] {
            1 => {
                let x = command[1];
                deque.push_front(x);
            }
            2 => {
                let x = command[1];
                deque.push_back(x);
            }
            3 => {
                if let Some(val) = deque.pop_front() {
                    writeln!(writer, "{}", val).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            4 => {
                if let Some(val) = deque.pop_back() {
                    writeln!(writer, "{}", val).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            5 => {
                writeln!(writer, "{}", deque.len()).unwrap();
            }
            6 => {
                if deque.is_empty() {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            }
            7 => {
                if let Some(&val) = deque.front() {
                    writeln!(writer, "{}", val).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            8 => {
                if let Some(&val) = deque.back() {
                    writeln!(writer, "{}", val).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            _ => unreachable!(),
        }
    }

    writer.flush().unwrap();
}
