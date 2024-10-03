use std::io::{self, BufRead, Write, BufWriter, stdin};

fn input_user_to_vec() -> Vec<i32> {
    let stdin = stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn move_tower(result: &mut Vec<String>, a: i32, b: i32) {
    result.push(format!("{} {}", a + 1, b + 1));
}

fn hanoi(result: &mut Vec<String>, a: i32, from: i32, k: i32, to: i32) {
    if a == 1 {
        move_tower(result, from, to);
    } else {
        hanoi(result, a - 1, from, to, k);
        move_tower(result, from, to);
        hanoi(result, a - 1, k, from, to);
    }
}

fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];
    let mut result: Vec<String> = Vec::new();
    hanoi(&mut result, n, 0, 1, 2);

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", result.len()).unwrap();
    for line in result {
        writeln!(writer, "{}", line).unwrap();
    }
}
