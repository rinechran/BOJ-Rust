use std::io;


fn input_number() -> i32{
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let number : i32 = input
        .trim()
        .parse()
        .unwrap();
    number
}

fn input_numbers() -> Vec<i32>{
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let number : Vec<i32> = input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    number
}
fn main() {
    let input_line = input_numbers();
    let nc = input_line[0];
    let want_val = input_line[1];

    let mut cards = input_numbers();

    let mut min_val = 0;
    'outer: for i in 0..cards.len(){
        for j in i+1..cards.len(){
            for k in j+1..cards.len(){
                let temp = cards[i] + cards[j] + cards[k];
                if temp == want_val{
                    min_val = want_val;
                    break 'outer;
                }
                if temp < want_val{
                    min_val = min_val.max(temp);
                }
            }
        }
    }

    println!("{}",min_val);

}
