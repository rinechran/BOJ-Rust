use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn w(arr: &mut HashMap<(i32,i32,i32),i32> , a: i32 , b : i32 , c: i32) -> i32{

    if let Some(&value) = arr.get(&(a, b, c)) {
        return value;
    }
    if a<=0 || b<=0 || c<=0 {
        return 1
    }
    if a>20 || b>20 || c>20 {
        return w(arr,20,20,20);
    }
    let result = if a < b && b < c {
        w(arr, a, b, c - 1) + w(arr, a, b - 1, c - 1) - w(arr, a, b - 1, c)
    } else {
        w(arr, a - 1, b, c) + w(arr, a - 1, b - 1, c) + w(arr, a - 1, b, c - 1) - w(arr, a - 1, b - 1, c - 1)
    };
    arr.insert((a, b, c), result);
    result
}

fn main() {
    let mut hash : HashMap<(i32,i32,i32),i32> = HashMap::new();
    loop {
        let mut input : Vec<i32> = input_user_to_vec();
        if input[0] == -1 && input[1] ==-1 && input[2] ==-1 {
            break;
        }
        let result = w(&mut hash,input[0],input[1],input[2]);
        println!("w({}, {}, {}) = {}",input[0],input[1],input[2],result);
    }

}
