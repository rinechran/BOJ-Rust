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
    let mut array = Vec::new();
    for _ in 0..7{
        let num: i32 = input_user_to_vec()[0];
        array.push(num);
    }
    let mut pick : Vec<i32>= array
        .into_iter()
        .filter(|&x| x % 2 != 0)
        .collect();
    if pick.len() == 0{
        println!("{}",-1);
    }
    else{
        pick.sort();
        let total: i32 = pick.iter().sum();
        println!("{}", total);
        println!("{}",pick[0]);
    }

}
