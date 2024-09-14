use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

fn is_point_in_circle(circle: &Circle, px: f32, py: f32) -> bool {
    let distance_squared = (px - circle.x).powi(2) + (py - circle.y).powi(2);
    distance_squared < circle.r.powi(2)
}

fn main() {
    let t: i32 = input_user_to_vec::<i32>()[0];

    for _ in 0..t {
        let mut total = 0;
        let row = input_user_to_vec::<f32>();
        let (start_x, start_y, dst_x, dst_y) = (row[0], row[1], row[2], row[3]);

        let circle_count: i32 = input_user_to_vec::<i32>()[0]; 

        let mut circles: Vec<Circle> = Vec::new();

        for _ in 0..circle_count {
            let row = input_user_to_vec::<f32>();
            let (x, y, r) = (row[0], row[1], row[2]);
            circles.push(Circle { x, y, r });
        }

        for circle in circles.iter() {
            let start_in_circle = is_point_in_circle(circle, start_x, start_y);
            let dst_in_circle = is_point_in_circle(circle, dst_x, dst_y);

            if start_in_circle && !dst_in_circle {
                total += 1; 
            } else if !start_in_circle && dst_in_circle {
                total += 1; 
            }
        }

        println!("{}", total); 
    }
}
