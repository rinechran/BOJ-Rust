use std::io;

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let result = (1..n).find(|&i| digit_sum(i) == n).unwrap_or(0);

    println!("{}", result);
}
