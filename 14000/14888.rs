use std::cmp::{min, max};
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
enum Operator {
    Plus,
    Minus,
    Multiply,
    Div,
}

#[derive(Clone)]
struct Status {
    input: Vec<i128>,
    use_operator: Vec<Operator>,
    plus: i32,
    minus: i32,
    multiply: i32,
    div: i32,
}

fn solve_operator(status: &Status) -> i128 {
    let mut input_iter = status.input.iter().skip(1);
    let mut operators_iter = status.use_operator.iter();
    let mut result = status.input[0];

    while let (Some(num), Some(op)) = (input_iter.next(), operators_iter.next()) {
        match op {
            Operator::Plus => result += num,
            Operator::Minus => result -= num,
            Operator::Multiply => result *= num,
            Operator::Div => result /= num
        }
    }

    result
}

fn solve(input: Status) -> (i128, i128) {
    let mut stack: Vec<Status> = vec![input];
    let mut max_value = i128::MIN;
    let mut min_value = i128::MAX;

    while let Some(status) = stack.pop() {
        if status.use_operator.len() == status.input.len() - 1 {
            let result = solve_operator(&status);
            min_value = min(min_value, result);
            max_value = max(max_value, result);
            continue;
        }

        if status.div > 0 {
            let mut clone = status.clone();
            clone.div -= 1;
            clone.use_operator.push(Operator::Div);
            stack.push(clone);
        }
        if status.minus > 0 {
            let mut clone = status.clone();
            clone.minus -= 1;
            clone.use_operator.push(Operator::Minus);
            stack.push(clone);
        }
        if status.multiply > 0 {
            let mut clone = status.clone();
            clone.multiply -= 1;
            clone.use_operator.push(Operator::Multiply);
            stack.push(clone);
        }
        if status.plus > 0 {
            let mut clone = status.clone();
            clone.plus -= 1;
            clone.use_operator.push(Operator::Plus);
            stack.push(clone);
        }
    }

    (max_value, min_value)
}

fn main() {
    let input: Vec<i128> = input_user_to_vec();
    let input: Vec<i128> = input_user_to_vec();
    let operator_input: Vec<i32> = input_user_to_vec();

    let status = Status {
        input,
        use_operator: vec![],
        plus: operator_input[0],
        minus: operator_input[1],
        multiply: operator_input[2],
        div: operator_input[3],
    };

    let result = solve(status);
    println!("{}\n{}", result.0, result.1);
}
