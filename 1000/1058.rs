use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {
    let friend_count : usize  = input_user_to_vec()[0];
    let mut friend_map: Vec<Vec<char>> = Vec::new();

    for _ in 0..friend_count {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();
        let row: Vec<char> = input_string
            .trim()
            .chars()
            .collect();
        friend_map.push(row);
    }

    let mut max_friend = 0;
    for i in 0..friend_count {
        let mut two_friend = vec![false; friend_count];
        for j in 0..friend_count {
            if friend_map[i][j] == 'Y' {
                two_friend[j] = true;
                for k in 0..friend_count {
                    if friend_map[j][k] == 'Y' && k != i {
                        two_friend[k] = true;
                    }
                }
            }
        }

        let count = two_friend
            .iter()
            .filter(|&&x| x)
            .count();
        max_friend = max_friend.max(count);
    }
    println!("{}", max_friend);
}
