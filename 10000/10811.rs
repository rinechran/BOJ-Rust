use std::io;
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let basket: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let N = basket[0];
    let M = basket[1];

    let mut array: Vec<usize> = (0..=N).collect();

    for _ in 0..M {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let line: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        let start_index = line[0];
        let end_index = line[1];

        array[start_index..=end_index].reverse();
    }

    for i in 1..=N {
        print!("{} ", array[i]);
    }
}
