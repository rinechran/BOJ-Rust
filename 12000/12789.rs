use std::collections::VecDeque;
use std::io;
use std::io::stdin;
use std::str::FromStr;

fn input_user_to_vec<T : FromStr > () -> Vec<T>{
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let row : Vec<i32> = input_user_to_vec();
    let n = row[0];
    let mut waitStack : Vec<i32> = Vec::new();
    let array : Vec<i32> = input_user_to_vec();
    let mut array : VecDeque<i32> = VecDeque::from(array);

    let mut want_index = 1;


    while let Some(front) = array.pop_front(){
        if front == want_index{
            want_index+=1;
        } else{
            waitStack.push(front);
        }
        while let Some(&top) = waitStack.last() {
            if top == want_index {
                waitStack.pop();
                want_index += 1;
            } else {
                break;
            }
        }
    }

    while let Some(top) = waitStack.pop() {
        if top == want_index {
            want_index += 1;
        } else {
            println!("Sad");
            return;
        }
    }
    println!("Nice");

}
