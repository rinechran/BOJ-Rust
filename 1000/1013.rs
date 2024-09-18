use std::io::{self, Write};

fn check_pattern(ptn: &str) -> bool {
    let mut start = 0; 
    let length = ptn.len();
    let mut flag;

    let pattern1 = "100"; 
    let pattern2 = "01";  

    let chars: Vec<char> = ptn.chars().collect(); 

    while start < length {
        if start + 2 < length && &ptn[start..start + 3] == pattern1 {
            start += 3;
            flag = 0;

            while start < length && chars[start] == '0' {
                start += 1;
            }

            while start < length && chars[start] == '1' {
                start += 1;
                flag += 1; 
            }

            if flag == 0 {
                return false; 
            }

            if start + 1 < length && chars[start + 1] == '0' && flag >= 2 {
                start -= 1;
            }
        }
        else if start + 1 < length && &ptn[start..start + 2] == pattern2 {
            start += 2;
        }
        else {
            return false;
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // 테스트 케이스 수 입력

    for _ in 0..t {
        let mut pattern = String::new();
        stdin.read_line(&mut pattern).unwrap();
        let pattern = pattern.trim();

        // 패턴 검사
        if check_pattern(pattern) {
            writeln!(stdout, "YES").unwrap();
        } else {
            writeln!(stdout, "NO").unwrap();
        }
    }
}
