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


fn get_percent(x: isize, y: isize) -> isize {
    (y * 100) / x
}
fn solve(x : isize, y : isize) -> isize{
    let initial_z  = get_percent(x,y);

    let mut result = -1;
    let mut left = 1;
    let mut right = 1_000_000_000;

    while left <= right {

        let mid = ( left + right ) / 2;
        let new_z = get_percent(x + mid, y + mid);
        if new_z > initial_z{
            result = mid;
            right = mid - 1;
        }else{
            left = mid + 1;
        }
    }

    return result ;
}
fn main() {
    let input: Vec<isize> = input_user_to_vec();
    let x = input[0];
    let y = input[1];

    let result = solve(x,y);
    println!("{}",result);

}
