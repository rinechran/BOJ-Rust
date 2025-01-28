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

fn main() {
    let nc: Vec<usize> = input_user_to_vec(); 
    let n = nc[0];
    let m = nc[1];

    let mut map: Vec<Vec<u32>> = Vec::new();
    for _ in 0..n {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        let row: Vec<u32> = input_string
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        map.push(row);
    }

    let mut max_size = 1; 

    for y1 in 0..n {
        for x1 in 0..m {
            for size in 1..=(n.max(m)) {
                let y2 = y1 + size;
                let x2 = x1 + size;

                if y2 >= n || x2 >= m {
                    break;
                }

                if map[y1][x1] == map[y1][x2]
                    && map[y1][x1] == map[y2][x1]
                    && map[y1][x1] == map[y2][x2]
                {
                    max_size = max_size.max((size + 1) * (size + 1));
                }
            }
        }
    }

    println!("{}", max_size); 
}
