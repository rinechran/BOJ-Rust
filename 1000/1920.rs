use std::collections::HashSet;
use std::io::{self, BufRead, BufWriter, Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock().lines();

    let n: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();

    let a: HashSet<i32> = reader
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();

    let targets: Vec<i32> = reader
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for target in targets {
        if a.contains(&target) {
            writeln!(writer, "1").unwrap();
        } else {
            writeln!(writer, "0").unwrap();
        }
    }
}
