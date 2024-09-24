use std::io::{self, BufRead};

fn is_vps(input: &str) -> &str {
    let mut stack = Vec::new();
    for ch in input.chars() {
        if ch == '(' {
            stack.push(ch);
        } else {
            if stack.is_empty() {
                return "NO";
            }
            stack.pop();
        }
    }
    if stack.is_empty() {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let result = is_vps(&line.trim());
            println!("{}", result);
        }
    }
}
