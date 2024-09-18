extern crate core;

use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write, StdinLock};

fn merge_sort_(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort_(&mut arr[..mid]);
    merge_sort_(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(len);
    let (mut left, mut right) = (0, mid);

    while left < mid && right < len {
        if arr[left] <= arr[right] {
            merged.push(arr[left]);
            left += 1;
        } else {
            merged.push(arr[right]);
            right += 1;
        }
    }

    if left < mid {
        merged.extend_from_slice(&arr[left..mid]);
    } else {
        merged.extend_from_slice(&arr[right..len]);
    }
    arr.copy_from_slice(&merged);
}

fn input_number(reader: &mut BufReader<StdinLock>) -> i32 {
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let test_case = input_number(&mut reader);

    let mut array: Vec<i32> = Vec::with_capacity(test_case as usize);

    for _ in 0..test_case {
        let input = input_number(&mut reader);
        array.push(input);
    }

    merge_sort_(&mut array);

    for i in array {
        writeln!(writer, "{}", i).unwrap();
    }

    writer.flush().unwrap();
}
