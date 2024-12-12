fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let _: Vec<isize> = input_user_to_vec();
    let mut array_1: Vec<isize> = input_user_to_vec();
    let mut array_2: Vec<isize> = input_user_to_vec();

    array_1.sort();
    array_2.sort_by(|a,b| b.cmp(a));


    let mut result = 0;

    for i in 0..array_1.len(){
        result+=array_1[i]*array_2[i];
    }
    println!("{}",result);

}
