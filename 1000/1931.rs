use std::io;

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct MeetingTime {
    start: usize,
    end: usize,
}

fn main() {
    let row: Vec<usize> = input_user_to_vec();
    let n = row[0];

    let mut meet_times: Vec<MeetingTime> = Vec::new();

    for _ in 0..n {
        let row: Vec<usize> = input_user_to_vec();
        let meet_time = MeetingTime {
            start: row[0],
            end: row[1],
        };
        meet_times.push(meet_time);
    }

    meet_times.sort_by(|a, b| {
        if a.end == b.end {
            a.start.cmp(&b.start)
        } else {
            a.end.cmp(&b.end)
        }
    });

    let mut count = 0;
    let mut last_end_time = 0;

    for meet_time in meet_times {
        if meet_time.start >= last_end_time {
            count += 1;
            last_end_time = meet_time.end;
        }
    }

    println!("{}", count);
}
