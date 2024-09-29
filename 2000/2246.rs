use std::collections::VecDeque;
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
    let first_input: Vec<usize> = input_user_to_vec();
    let n = first_input[0];

    let papers: Vec<i32> = input_user_to_vec();

    let mut balloons: VecDeque<(usize, i32)> = (0..n).map(|i| (i + 1, papers[i])).collect();
    let mut result: Vec<usize> = Vec::new();

    while balloons.is_empty() != true {
        let front = balloons
            .pop_front()
            .unwrap();

        result.push(front.0);

        if balloons.is_empty() {
            break;
        }

        let move_val = front.1;
        if move_val >=0{
            for _ in 0..(move_val - 1) {
                let temp = balloons.pop_front().unwrap();
                balloons.push_back(temp);
            }
        }else {
            for _ in 0..(-move_val) {
                let temp = balloons.pop_back().unwrap();
                balloons.push_front(temp);
            }
        }
    }
    let result : Vec<String>= result
        .iter()
        .map(|x| x.to_string())
        .collect();
    let result = result.join(" ");

    println!("{}",result);
}
