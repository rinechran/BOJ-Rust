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
    let row: Vec<isize> = input_user_to_vec();
    let mut money = row[0];

    money = 1000-money;
    let remain_moneys = [ 500, 100, 50, 10, 5, 1];
    let mut total = 0;

    for remain_money in remain_moneys{
        let k = money/remain_money;
        money-=remain_money*k;
        total +=k;
    }
    println!("{}",total);

}
