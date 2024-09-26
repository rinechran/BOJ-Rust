use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lcm(a: i128, b: i128) -> i128 {
    (a * b) / gcd(a, b)
}

fn main() {
    let row: Vec<i128> = input_user_to_vec();
    let n_1 = row[0];
    let m_1 = row[1];

    let row: Vec<i128> = input_user_to_vec();
    let n_2 = row[0];
    let m_2 = row[1];

    let common_denominator = lcm(m_1, m_2);

    let numerator1 = n_1 * (common_denominator / m_1);
    let numerator2 = n_2 * (common_denominator / m_2);
    let sum_numerator = numerator1 + numerator2;

    let gcd_value = gcd(sum_numerator, common_denominator);
    let final_numerator = sum_numerator / gcd_value;
    let final_denominator = common_denominator / gcd_value;

    println!("{} {}", final_numerator, final_denominator);
}
