use std::io::stdin;

static mut COUNTER: i32 = 0;
fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}
fn init_counter() {
    unsafe {
        COUNTER = 0;
    }
}
fn get_counter() -> i32 {
    let mut count = 0;
    unsafe {
        count = COUNTER;
    }
    return count;
}
fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn recursion(s : &str,l : usize , r: usize) -> i32{
    increment_counter();
    if l >= r{
        return 1;
    }
    else if s.as_bytes()[l] != s.as_bytes()[r] {
        return 0;
    }
    return recursion(s, l+1, r-1);
}
fn is_palindrome(str : String ) -> i32{
    init_counter();
    return recursion(&str, 0, str.len() - 1);
}
fn main() {
    let row: Vec<i32> = input_user_to_vec();
    let n = row[0];
    for _ in 0..n{
        let row: Vec<String> = input_user_to_vec();
        let is_palindrome_val = is_palindrome(row[0].clone());
        let result_COUNTER = get_counter();
        println!("{} {}",is_palindrome_val,result_COUNTER);
    }

}
