use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {

    let _ : Vec<usize> = input_user_to_vec();
    let mut human_wait_time : Vec<usize> = input_user_to_vec();
    human_wait_time.sort();

    let mut total_wait_time = 0;
    let mut prefix_sum = 0;

    for time in human_wait_time {
        prefix_sum += time;
        total_wait_time += prefix_sum;
    }

    println!("{}", total_wait_time);
}
