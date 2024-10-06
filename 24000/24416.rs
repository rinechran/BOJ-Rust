use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

use std::io;

static mut RECURSIVE_COUNT: i32 = 0;
static mut DP_COUNT: i32 = 0;

fn fib_recursive(n: usize) -> usize {

    if n == 1 || n == 2 {
        return 1;
    } else {
        unsafe {
            RECURSIVE_COUNT += 1;
        }
        return fib_recursive(n - 1) + fib_recursive(n - 2);
    }
}

fn fib_dynamic(n: usize) -> usize {
    let mut f = vec![0; n + 1];
    f[1] = 1;
    f[2] = 1;

    for i in 3..=n {
        unsafe {
            DP_COUNT += 1;
        }
        f[i] = f[i - 1] + f[i - 2];
    }

    f[n]
}

fn main() {
    let mut input : Vec<usize> = input_user_to_vec();

    unsafe {
        fib_recursive(input[0]);
        fib_dynamic(input[0]);

        println!("{} {}", RECURSIVE_COUNT+1, DP_COUNT);
    }
}
