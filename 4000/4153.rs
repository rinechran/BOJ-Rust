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
    loop {
        let mut row : Vec<i32> = input_user_to_vec();
        row.sort();

        if row.iter().all(|&x| x==0) == true{
            break;
        }

        if row[0]*row[0] + row[1]*row[1] == row[2]*row[2]{
            println!("right");
        }else{
            println!("wrong");
        }

    }

}
