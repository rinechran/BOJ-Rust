use std::cmp::Reverse;
use std::io::stdin;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct Matrix{
    row : usize,
    col : usize,
    data : Vec<Vec<i32>>
}
impl Matrix{
    fn new(data : Vec<Vec<i32>>) -> Matrix {
        let row = data.len();
        let col = data[0].len();
        Matrix {
            row,
            col,
            data
        }
    }
    fn multiply(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.col != other.row{
            return Err("Err");
        }

        let mut result_data = vec![vec![0; other.col]; self.row];
        for i in 0..self.row {
            for j in 0..other.col {
                for k in 0..self.col {
                    result_data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Ok(Matrix::new(result_data))
    }
}

fn new_matrix() -> Matrix{
    let size: Vec<usize> = input_user_to_vec();
    let N = size[0];
    let mut data = Vec::new();
    for _ in 0..N {
        let row: Vec<i32> = input_user_to_vec();
        data.push(row);
    }
    Matrix::new(data)
}
fn main() {

    let m = new_matrix();
    let n = new_matrix();

    let result = m.multiply(&n);
    match m.multiply(&n) {
        Ok(matrix) => {
            for row in matrix.data {
                for col in row {
                    print!("{} ", col);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }


}