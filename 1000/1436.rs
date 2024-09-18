use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn check_three_six(s: String) -> bool{

    if s.len() <3{
        return false;
    }

    for i in 0..(s.len() - 2) {
        if &s[i..i + 3] == "666" {
            return true;
        }
    }

    return false;

}

fn main() {
    let want_level: i32= input_user_to_vec()[0];


    let mut current_level = 0;
    for i in 666..1000000000{
        if check_three_six(i.to_string()) == true{
            current_level+=1;
            if current_level==want_level{
                println!("{}",i);
                return;
            }
        }

    }
}
