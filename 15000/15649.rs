use std::io::{self, BufRead, Write, BufWriter, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn permute<W: Write>(arr: &Vec<i32>, selected: &mut Vec<i32>, r: i32, output: &mut W) {
    if selected.len() == r as usize {
        for r in 0..selected.len() {
            write!(output, "{} ", selected[r]).unwrap();
        }
        writeln!(output).unwrap();
        return;
    }
    for index in 0..arr.len() {
        if selected.contains(&arr[index]) {
            continue;
        }
        selected.push(arr[index]);
        permute(arr, selected, r, output);
        selected.pop();
    }
}

fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];
    let k = row[1];
    let arr: Vec<i32> = (1..=n).collect();
    let mut selected: Vec<i32> = Vec::new();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    permute(&arr, &mut selected, k, &mut writer);
}
