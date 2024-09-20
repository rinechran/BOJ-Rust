use std::io;
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let dimensions: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let n = dimensions[0];
    let m = dimensions[1];

    let mut baskets: Vec<usize> = vec![0; n]; // 초기 바구니 상태 (0으로 초기화)

    for i in 0..n{
        baskets[i]=i+1;
    }
    for i in 0..m{
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let dimensions: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        let startIndex = dimensions[0]-1;
        let endIndex = dimensions[1]-1;
        let mut temp = baskets[startIndex];

    
        baskets.swap(startIndex,endIndex);
    }
    for i in &baskets{
        print!("{} ",i);
    }
}
