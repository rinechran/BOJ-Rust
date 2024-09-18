use std::io;

fn sugar_delivery(n: i32) -> i32 {
    let mut n = n;
    let mut bag_count = 0;

    while n >= 0 {
        if n % 5 == 0 {
            bag_count += n / 5;
            return bag_count;
        }
        n -= 3;
        bag_count += 1;
    }

    -1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let result = sugar_delivery(n);
    println!("{}", result);
}
