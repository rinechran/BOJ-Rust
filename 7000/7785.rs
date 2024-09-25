use std::collections::BTreeMap;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout());
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().trim().parse().unwrap();

    let mut s_map : BTreeMap<String,i32> = BTreeMap::new();

    for _ in 0..n {
        let line = input.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let command = iter.next().unwrap();

        match command {
            "enter" => {
                *s_map.entry(name).or_default() += 1;
            }
            "leave" => {
                if let Some(count) = s_map.get_mut(&name) {
                    *count -= 1;
                    if *count == 0 {
                        s_map.remove(&name);
                    }
                }
            }
            _ => {}
        }
    }

    for name in s_map.keys().rev() {
        writeln!(stdout, "{}", name).unwrap();
    }
    stdout.flush().unwrap();
}
