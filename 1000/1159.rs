use std::collections::HashMap;
use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {

    let nc : i32 = input_user_to_vec()[0];

    let mut frequency : HashMap<char,i32> = HashMap::new();
    for _ in 0..nc{
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        let first_ch : char = input_string
            .trim()
            .chars()
            .next()
            .unwrap();
        *frequency.entry(first_ch).or_insert(0) +=1;
    }

    let mut result: Vec<char> = frequency
        .iter()
        .filter(|&(_, &value)| value >= 5) 
        .map(|(&key, _)| key) 
        .collect();

    if result.is_empty() {
        println!("PREDAJA");
    } else {
        result.sort();
        for ch in result {
            print!("{}", ch);
        }
    }
}
