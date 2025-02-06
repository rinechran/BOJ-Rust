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

fn solve(mut val : i32) {

    let mut index = 0;
    while val!=0 {
        let mo = val % 2;
        if mo == 1{
            print!("{} ",index);
        }
        index= index+1;
        val = val /2;
    }
}
fn main() {

    let nc : i32 = input_user_to_vec()[0];
    for _ in 0..nc{
        let number : i32 = input_user_to_vec()[0];
        solve(number);
        println!();
    }

}
