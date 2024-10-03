use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut result = Vec::new();
    let mut input_string = String::new();

    while stdin().read_line(&mut input_string).unwrap() > 0 {
        let values: Vec<T> = input_string
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<T>().ok())
            .collect();
        result.extend(values);
        input_string.clear();
    }

    result
}

fn cantor_set(a: &mut Vec<char>, l: usize, r: usize) {
    if r - l < 3 {
        return;
    }

    let len = (r - l) / 3;

    for i in l + len..l + 2 * len {
        a[i] = ' ';
    }

    cantor_set(a, l, l + len);
    cantor_set(a, l + 2 * len, r);
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();

    for number in row {
        let pow_value = 3_usize.pow(number as u32);
        let mut result_string: Vec<char> = vec!['-'; pow_value];
        let result_len = result_string.len();
        cantor_set(&mut result_string, 0, result_len);
        let result: String = result_string.into_iter().collect();
        println!("{}", result);
    }
}
