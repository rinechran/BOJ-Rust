use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn is_prime(n: i128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0  {
            return false;
        }
        i +=1;
    }
    true
}

fn main() {
    let row: Vec<i128> = input_user_to_vec();
    let n = row[0];

    for _ in 0..n{
        let row: Vec<i128> = input_user_to_vec();
        let mut num = row[0];
        loop{
            if is_prime(num) == true{
                println!("{}",num);
                break;
            }
            num+=1;
        }
    }
}
