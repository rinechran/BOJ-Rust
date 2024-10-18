use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let open_windows = (n as f64).sqrt() as u64;

    println!("{}", open_windows);
}
