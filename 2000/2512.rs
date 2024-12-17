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


fn solve(array : Vec<isize> ,expect : isize) -> isize{

    let mut left = 1;
    let mut right = array.iter().max().unwrap().clone();
    let mut result = -1;

    while left<=right{
        let mid = (left + right)/2;
        let sum : isize = array.iter()
            .map(|&s| s.min(mid))
            .sum();

        if sum <= expect{
            left = mid + 1;
            result = mid;
        }
        else{
            right = mid -1;
        }
    }

    return result ;
}
fn main() {
    let _: Vec<isize> = input_user_to_vec();
    let array: Vec<isize>  = input_user_to_vec();
    let expect : isize = input_user_to_vec()[0];

    let result = solve(array,expect);
    println!("{}",result);

}
