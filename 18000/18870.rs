use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn read_input_vec<T: std::str::FromStr>(reader: &mut BufReader<io::StdinLock>) -> Vec<T> {
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();

    input_line
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let _: i32 = read_input_vec(&mut reader)[0];

    let input_values: Vec<i32> = read_input_vec(&mut reader);

    let unique_values: BTreeSet<i32> = input_values.clone().into_iter().collect();

    let mut value_index_map: HashMap<i32, usize> = HashMap::new();
    for (index, value) in unique_values.into_iter().enumerate() {
        value_index_map.insert(value, index);  
    }

    for value in input_values {
        if let Some(&index) = value_index_map.get(&value) {
            write!(writer, "{} ", index).unwrap();
        }
    }

    writer.flush().unwrap();
}
