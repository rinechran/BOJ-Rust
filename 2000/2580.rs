use std::io::stdin;

const SIZE : usize = 9;
type Board = [[i32; SIZE]; SIZE];

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
#[derive(Clone)]
struct Status {
    row : usize,
    col : usize,
    board : Board
}

fn is_valid(status: &Status, row: usize, col: usize, n: i32) -> bool {
    for i in 0..SIZE {
        if status.board[row][i] == n as i32 {
            return false;
        }

        if status.board[i][col] == n as i32 {
            return false;
        }

        let box_row = (row / 3) * 3 + i / 3;
        let box_col = (col / 3) * 3 + i % 3;
        if status.board[box_row][box_col] == n as i32 {
            return false;
        }
    }
    true
}

fn solve_sudoku(
    board: &mut Board
)-> bool{

    let mut stack = vec![Status{
        row: 0,
        col: 0,
        board: *board,
    }];

    while let Some(mut status) = stack.pop(){
        if status.row == SIZE{
            *board = status.board;
            return true;
        }

        let (next_row, next_col) = if status.col == SIZE - 1 {
            (status.row + 1, 0)
        } else {
            (status.row, status.col + 1)
        };

        if status.board[status.row][status.col] != 0 {
            stack.push(Status {
                row: next_row,
                col: next_col,
                board: status.board,
            });
            continue;
        }

        for n in 1..=SIZE as i32{
            if is_valid(&status, status.row, status.col ,n) == true{
                let mut new_status = status.clone();
                new_status.board[status.row][status.col] = n;
                new_status.row = next_row;
                new_status.col = next_col;
                stack.push(new_status);
            }
        }
    }

    return false;
}

fn main() {
    let mut board: Board = [[0; SIZE]; SIZE];

    for i in 0..9{
        let row : Vec<i32> = input_user_to_vec();
        for j in 0..9{
            board[i][j] = row[j];
        }
    }
    solve_sudoku(&mut board);
    for row in &board {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }

}