use std::cmp::min;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn combinations(arr: Vec<i128>, k: usize) -> Vec<Vec<i128>> {
    let mut result: Vec<Vec<i128>> = Vec::new();

    fn combine(arr: &Vec<i128>, data: &mut Vec<i128>, current_sel: usize, k: usize, start: usize, result: &mut Vec<Vec<i128>>) {
        if current_sel == k {
            result.push(data.clone());
            return;
        }
        for index in start..arr.len() {
            data[current_sel] = arr[index].clone();
            combine(arr, data, current_sel + 1, k, index + 1, result);
        }
    }

    let mut data = vec![0; k];
    combine(&arr, &mut data, 0, k, 0, &mut result);

    result
}
fn solve(team: &Vec<i128>, board: &Vec<Vec<i128>>) -> i128 {
    let mut result = 0;
    for i in 0..team.len() {
        for j in 0..team.len() {
            result += board[team[i] as usize][team[j] as usize];
        }
    }
    result
}
fn main() {
    let input: Vec<i128> = input_user_to_vec();
    let team_size = input[0] as usize;
    let original_team: Vec<i128> = (0..team_size as i128).collect();
    let mut board: Vec<Vec<i128>> = Vec::new();

    for _ in 0..team_size {
        let input: Vec<i128> = input_user_to_vec();
        board.push(input);
    }

    let teams = combinations(original_team.clone(), team_size / 2);

    let mut diff = 10000000;
    for team in teams.iter() {
        let other_team: Vec<i128> = original_team
            .iter()
            .filter(|&&x| !team.contains(&x)) 
            .cloned() 
            .collect();

        let team1 = solve(&team,&board);
        let team2 = solve(&other_team,&board);

        let current_diff = (team1 - team2).abs();
        diff = min(diff,current_diff)
    }
    println!("{}",diff);

}
