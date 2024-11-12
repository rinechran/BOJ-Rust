use std::cmp::Reverse;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn is_uniform(paper:  &Vec<Vec<usize>> , x: usize, y: usize, size: usize) -> bool {
    let color = paper[x][y];
    for i in x..x+size{
        for j in y..y + size {
            if paper[i][j] != color {
                return false;
            }
        }
    }
    true
}

fn get_count_color_white_blue(paper : &Vec<Vec<usize>>, x: usize, y: usize, size: usize) -> (i32,i32){
    if is_uniform(paper, x, y, size) {
        if paper[x][y] == 1 {
            return (0, 1);
        } else {
            return (1, 0);
        }
    }

    let half = size / 2;
    let (w1,b1) = get_count_color_white_blue(paper,x,y,half);
    let (w2,b2) = get_count_color_white_blue(paper,x+half,y,half);
    let (w3,b3) = get_count_color_white_blue(paper,x,y+half,half);
    let (w4,b4) = get_count_color_white_blue(paper,x+half,y+half,half);

    return (w1+w2+w3+w4,b1+b2+b3+b4);
}
fn main() {
    let nc: Vec<usize> = input_user_to_vec();

    let mut map : Vec<Vec<usize>> = Vec::new();

    for _ in 0..nc[0]{
        let row: Vec<usize> = input_user_to_vec();
        map.push(row);
    }

    let (white,blue) = get_count_color_white_blue(&map,0,0,nc[0]);
    println!("{}",white);
    println!("{}",blue);
}
