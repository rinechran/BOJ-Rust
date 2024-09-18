use std::io;

fn main() {
    // 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 
    let mut seats = String::new();
    io::stdin().read_line(&mut seats).unwrap(); 

    let seats = seats.trim();

    let mut holder_count = 1; 

    let mut i = 0;
    while i < seats.len() {
        if &seats[i..i+1] == "S" {
            holder_count += 1; 
            i += 1;
        } else if &seats[i..i+2] == "LL" {
            holder_count += 1; 
            i += 2;
        }
    }
    

    let max_people = holder_count.min(seats.len());

    println!("{}", max_people);
}
