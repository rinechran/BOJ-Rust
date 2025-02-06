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

    let nc : Vec<i32> = input_user_to_vec();
    for _ in 0..nc[0] {
        let mut arr : Vec<i32> = input_user_to_vec();
        arr.sort_by(|a,b| b.cmp(a));
        println!("{}",arr[2]);
    }

}
