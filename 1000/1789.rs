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

    let want : u64 = input_user_to_vec()[0];

    let mut sum_acl: u64 = 0;
    let mut i: u64 = 0;

    while sum_acl + (i + 1) <= want {
        i += 1;
        sum_acl += i;
    }

    println!("{}", i);

}
