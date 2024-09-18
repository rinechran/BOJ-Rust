use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let test_case: usize = input.trim().parse().unwrap();

    let mut array: Vec<i32> = vec![0; 10001];

    for _ in 0..test_case {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let num: usize = input.trim().parse().unwrap();
        array[num] += 1;
    }

    for i in 1..=10000 {
        for _ in 0..array[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }

    writer.flush().unwrap();
}
