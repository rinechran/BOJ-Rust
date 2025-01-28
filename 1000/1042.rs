use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn calculate_min_visible_sum(n: u64, dice: &[u64]) -> u64 {
    let (a, b, c, d, e, f) = (dice[0], dice[1], dice[2], dice[3], dice[4], dice[5]);

    let one_face_min = *[a, b, c, d, e, f].iter().min().unwrap();
    let two_face_min = *[
        a + b, a + c, a + d, a + e,
        b + c, b + d,
        e + c, e + d,
        f + b, f + c, f + d, f + e,
    ]
        .iter()
        .min()
        .unwrap();
    let three_face_min = *[
        a + b + c, a + b + d, a + e + c, a + e + d,
        f + b + c, f + b + d, f + e + c, f + e + d,
    ]
        .iter()
        .min()
        .unwrap();

    if n == 1 {
        return dice.iter().sum::<u64>() - dice.iter().max().unwrap();
    }

    let three_face_count = 4;
    let two_face_count = 4 * (n - 1) + 4 * (n - 2);
    let one_face_count = (n - 2).pow(2) + 4 * (n - 2) * (n - 1);

    three_face_count * three_face_min + two_face_count * two_face_min + one_face_count * one_face_min
}

fn main() {
    let n: u64 = {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        input_string.trim().parse().unwrap()
    };

    let dice: Vec<u64> = input_user_to_vec();

    let result = calculate_min_visible_sum(n, &dice);
    println!("{}", result);
}
