use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {

    let mut arr = [0usize; 102];

    let values = [1, 1, 1, 2, 2, 3, 4, 5, 7, 9];
    for (i, &val) in values.iter().enumerate() {
        arr[i] = val;
    }
    for i in 10..arr.len(){
        arr[i] = arr[i-2] + arr[i-3];
    }

    let input : Vec<i32> = input_user_to_vec();
    let nc = input[0];

    for _ in 0..nc  {
        let input : Vec<usize> = input_user_to_vec();
        let n = input[0];
        println!("{}",arr[n-1]);
    }

}
