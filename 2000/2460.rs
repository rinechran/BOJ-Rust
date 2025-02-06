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

    let mut arrs :Vec<(i32,i32)> = Vec::new();
    for _ in 0..10{
        let nc : Vec<i32> = input_user_to_vec();
        arrs.push((nc[0],nc[1]));
    }
    let mut current_total = 0;
    let mut max_index : i32 = -1;

    for arr in arrs.iter(){
        current_total = current_total + arr.1;
        current_total = current_total - arr.0;

        if max_index < current_total{
            max_index = current_total;
        }
    }
    println!("{}",max_index);


}
