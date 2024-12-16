use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .replace(","," ")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {
    let input: Vec<isize> = input_user_to_vec();
    let n = input[0] as usize;

    for _ in 0..n{
        let row: Vec<i32> = input_user_to_vec();
        println!("{}",row[0]+row[1]);
    }

}
