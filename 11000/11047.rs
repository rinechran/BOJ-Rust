use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {

    let input : Vec<i64> = input_user_to_vec();
    let count = input[0];
    let mut current_money = input[1];

    let mut total_case_count = 0;
    let mut menoy : Vec<i64> = Vec::new();
    for _ in 0..count{
        let input : Vec<i64> = input_user_to_vec();
        menoy.push(input[0]);
    }

    for sel_money in menoy.iter().rev(){
        let div_count = current_money /sel_money;
        total_case_count+=div_count;
        if div_count != 0{
            current_money -=div_count*sel_money;
        }
    }

    println!("{}",total_case_count);

}
