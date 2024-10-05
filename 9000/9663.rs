use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[derive(Clone)]
struct State {
    row: usize,
    cols: Vec<bool>,
    diags1: Vec<bool>,
    diags2: Vec<bool>,
}

fn n_queens(n: usize) -> u32 {
    let mut solutions = 0;
    let mut stack = vec![State {
        row: 0,
        cols: vec![false; n],
        diags1: vec![false; n * 2],
        diags2: vec![false; n * 2],
    }];

    while let Some(state) = stack.pop() {
        if state.row == n {
            solutions += 1;
            continue;
        }

        for col in 0..n {
            let diag1 = state.row + col;
            let diag2 = state.row + (n - 1) - col;
            if !state.cols[col] && !state.diags1[diag1] && !state.diags2[diag2] {
                let mut new_state = state.clone();
                new_state.cols[col] = true;
                new_state.diags1[diag1] = true;
                new_state.diags2[diag2] = true;
                new_state.row += 1;
                stack.push(new_state);
            }
        }
    }

    solutions
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let result = n_queens(row[0]);
    println!("{}", result);
}