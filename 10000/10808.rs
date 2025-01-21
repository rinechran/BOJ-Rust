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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let str_vec: Vec<char> = input.trim().chars().collect();

    let alphabet_size = ('z' as usize -'a' as usize) + 1;
    let mut alphabet_frequency = vec![0;alphabet_size];
    for i in str_vec{
        let index = (i.to_ascii_lowercase() as usize) - ('a' as usize);
        alphabet_frequency[index] += 1;
    }

    for i in alphabet_frequency.iter(){
        print!("{} ",i);
    }

}
