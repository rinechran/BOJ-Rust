use std::io::{self, stdin};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut numbers: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();  
        stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().unwrap();
        numbers.push(number);
    }

    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }

    for number in numbers {
        println!("{}", number);
    }
}
