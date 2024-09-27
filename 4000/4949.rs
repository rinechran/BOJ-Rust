use std::io::{self, BufRead, Write, BufWriter};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    loop {
        if let Some(Ok(row)) = input.next() {
            if row == "." {
                break;
            }

            let mut stack: Vec<char> = Vec::new();
            let mut valid = true;

            for ch in row.chars() {
                match ch {
                    '(' | '[' => {
                        stack.push(ch);
                    }
                    ')' => {
                        if let Some(top) = stack.pop() {
                            if top != '(' {
                                valid = false;
                                break;
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    }
                    ']' => {
                        if let Some(top) = stack.pop() {
                            if top != '[' {
                                valid = false;
                                break;
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    }
                    _ => continue,
                }
            }

            if valid && stack.is_empty() {
                writeln!(out, "yes").unwrap();
            } else {
                writeln!(out, "no").unwrap();
            }
        }
    }
    out.flush().unwrap();
}
