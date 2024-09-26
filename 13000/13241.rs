use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn gcd(mut a : i128 , mut b : i128) -> i128{
    while b!=0 {
        let mut c = a % b;
        a = b;
        b = c;
    }
    return a;
}
fn main() {

    let row : Vec<i128> = input_user_to_vec();
    let n = row[0];
    let m = row[1];
    let gcd_value = gcd(n,m);
    let result = (n*m)/gcd_value;
    println!("{}",result);
}
