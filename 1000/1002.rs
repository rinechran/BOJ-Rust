use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

enum Intersection {
    None,
    Infinite,
    One,
    Two,
}

fn find_circle_intersection(x1: f32, y1: f32, r1: f32, x2: f32, y2: f32, r2: f32) -> Intersection {
    let d = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    match () {
        _ if d == 0.0 && r1 == r2 => Intersection::Infinite,
        _ if d > r1 + r2 || d < (r1 - r2).abs() => Intersection::None,
        _ if d == r1 + r2 || d == (r1 - r2).abs() => Intersection::One,
        _ => Intersection::Two,
    }
}

fn main() {
    let n: i32 = input_user_to_vec::<i32>()[0];

    for _ in 0..n {
        let row = input_user_to_vec::<f32>();

        let (x1, y1, r1) = (row[0], row[1], row[2]);
        let (x2, y2, r2) = (row[3], row[4], row[5]);

        let result = find_circle_intersection(x1, y1, r1, x2, y2, r2);

        let result = match result {
            Intersection::None => 0,
            Intersection::Infinite => -1,
            Intersection::One => 1,
            Intersection::Two => 2,
        };
        // 교점 개수 출력
        println!("{}",result);
    }
}
