use std::collections::VecDeque;
use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


struct Center {
    towers: Vec<usize>,
    adj_list: Vec<Vec<usize>>,
    in_degree: Vec<usize>,
}

impl Center {
    fn new(tower_count: usize) -> Center {
        Center {
            towers: vec![0; tower_count],
            adj_list: vec![Vec::new(); tower_count],
            in_degree: vec![0; tower_count],
        }
    }
    fn set_tower_time(&mut self, index: usize, time: usize) {
        self.towers[index] = time;
    }
    fn add_dependency(&mut self, pre : usize ,post :usize){
        self.adj_list[pre].push(post);
        self.in_degree[post]+=1;
    }
    fn calculate_min_time(&self,target : usize) -> usize{
        let mut queue = VecDeque::new();
        let mut build_times= self.towers.clone();
        let mut in_degree = self.in_degree.clone();


        for (i, &deg) in self.in_degree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(i);
            }
        }

        while let Some(current) = queue.pop_front(){
            let current_time = build_times[current];

            for &next in &self.adj_list[current]{
                in_degree[next] -= 1;
                if build_times[next] < current_time + self.towers[next] {
                    build_times[next] = current_time + self.towers[next];
                }
                if in_degree[next] == 0 {
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
    let n: usize = input_user_to_vec()[0];

    for _ in 0..n {
        run();
    }
}
