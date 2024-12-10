use std::collections::{BinaryHeap, VecDeque};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
fn main() {
    let test_cases: usize = input_user_to_vec()[0];

    for _ in 0..test_cases {
        let params: Vec<usize> = input_user_to_vec();
        let n = params[0];
        let m = params[1];

        let priorities: Vec<i32> = input_user_to_vec();

        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
        let mut max_heap = BinaryHeap::new();

        for (index, &priority) in priorities.iter().enumerate() {
            queue.push_back((priority, index));
            max_heap.push(priority);
        }

        let mut print_order = 0;

        while let Some((priority,index)) = queue.pop_front(){
            if priority == *max_heap.peek().unwrap(){
                print_order += 1;
                max_heap.pop();
                if index == m {
                    println!("{}", print_order);
                    break;
                }
            }
            else{
                queue.push_back((priority,index));
            }
        }
    }
}