use std::collections::VecDeque;
use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[derive(Clone)]
struct Tower {
    build_time: i32,
}

struct Center {
    towers: Vec<Tower>,
    adj_list: Vec<Vec<usize>>,
    indegree: Vec<usize>,
}

impl Center {
    fn new(tower_count: usize) -> Center {
        Center {
            towers: vec![
                Tower {
                    build_time: 0,
                };
                tower_count
            ],
            adj_list: vec![Vec::new(); tower_count],
            indegree: vec![0; tower_count],
        }
    }

    fn set_tower_time(&mut self, index: usize, time: i32) {
        self.towers[index].build_time = time;
    }

    fn add_dependency(&mut self, pre_tower: usize, post_tower: usize) {
        self.adj_list[pre_tower].push(post_tower);
        self.indegree[post_tower] += 1;
    }

    fn calculate_min_time(&self, target: usize) -> i32 {
        let mut build_times = vec![0; self.towers.len()];
        let mut indegree = self.indegree.clone();
        let mut queue = VecDeque::new();

        for (i, tower) in self.towers.iter().enumerate() {
            build_times[i] = tower.build_time;
        }

        for (i, &deg) in indegree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(i);
            }
        }

        while let Some(current) = queue.pop_front() {
            let current_time = build_times[current];

            for &next in &self.adj_list[current] {
                indegree[next] -= 1;
                if build_times[next] < current_time + self.towers[next].build_time {
                    build_times[next] = current_time + self.towers[next].build_time;
                }
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        build_times[target]
    }
}

fn run() {
    let rows = input_user_to_vec();
    let tower_count = rows[0] as usize;
    let rule_count = rows[1] as usize;

    let mut center = Center::new(tower_count);

    let times = input_user_to_vec();
    for (index, &time) in times.iter().enumerate() {
        center.set_tower_time(index, time);
    }

    for _ in 0..rule_count {
        let rule = input_user_to_vec();
        let pre = rule[0] as usize - 1;
        let post = rule[1] as usize - 1;
        center.add_dependency(pre, post);
    }

    let target_building = input_user_to_vec()[0] as usize - 1;
    let result = center.calculate_min_time(target_building);
    println!("{}", result);
}

fn main() {
    let n: i32 = input_user_to_vec()[0];

    for _ in 0..n {
        run();
    }
}
