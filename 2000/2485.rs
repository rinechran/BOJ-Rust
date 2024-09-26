use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}



fn main() {
    let row: Vec<i128> = input_user_to_vec();
    let n = row[0];
    let row: Vec<i128> = input_user_to_vec();
    let m =row[0];
    let mut arr : Vec<i128> = Vec::new();
    for _ in 0..n-1{
        let row: Vec<i128> = input_user_to_vec();
        let value = row[0] - m ;
        arr.push(value);
    }
    let last_val = arr[arr.len()-1];
    for i in 0..arr.len()-1{
        arr[i + 1] = gcd(arr[i],arr[i+1]);
    }

    let total_val = last_val/arr[arr.len()-1];
    println!("{}",total_val -(arr.len()  as i128));

}
