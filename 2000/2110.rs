use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn can_place_routers(houses: &Vec<usize>, want_router: usize, distance: usize) -> bool {
    let mut count = 1;
    let mut last_position = houses[0];

    for &house in houses.iter().skip(1) {
        if house - last_position >= distance {
            count += 1;
            last_position = house;
        }
        if count >= want_router {
            return true;
        }
    }

    false
}

fn router_max_min_distance(houses: &mut Vec<usize>, want_router: usize) -> usize {
    houses.sort();

    let mut left = 1;
    let mut right = houses[houses.len() - 1] - houses[0];
    let mut result = 0;

    while left <= right { 
        let mid = (left + right) / 2;

        if can_place_routers(houses, want_router, mid) {
            result = mid; 
            left = mid + 1;
        } else {
            right = mid - 1; 
        }
    }

    result
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let house_count = row[0];
    let router_count = row[1];

    let mut houses = Vec::new();

    for _ in 0..house_count {
        let row: Vec<usize> = input_user_to_vec();
        houses.push(row[0]);
    }

    let result = router_max_min_distance(&mut houses, router_count);
    println!("{}", result);
}
