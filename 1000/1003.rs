use std::io;

fn main() {
    let mut fib = vec![0; 42];
    fib[0] = 1;
    fib[1] = 0;

    for i in 2..42 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        println!("{} {}", fib[n], fib[n + 1]);
    }
}
