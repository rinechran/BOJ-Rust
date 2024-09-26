use std::io::stdin;

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;

    for p in 2..=((limit as f64).sqrt() as usize) {
        if primes[p] {
            let mut multiple = p + p;
            while multiple <= limit {
                primes[multiple] = false;
                multiple += p;
            }
        }
    }
    primes
}

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let primes = sieve_of_eratosthenes(2 * n);

        let count = (n + 1..=2 * n)
            .filter(|&x| primes[x])
            .count();

        println!("{}", count);
    }
}
