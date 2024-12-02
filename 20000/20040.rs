use std::io::{self, stdin};

fn input_user_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct VecUnionFind {
    size: usize,
    data: Vec<usize>,
}

impl VecUnionFind {
    fn new(size: usize) -> VecUnionFind {
        VecUnionFind {
            size,
            data: (0..size).collect(),
        }
    }

    fn find(&mut self, key: usize) -> usize {
        if self.data[key] != key {
            self.data[key] = self.find(self.data[key]);
        }
        self.data[key]
    }

    fn merge(&mut self, parent: usize, child: usize) -> bool {
        let parent_root = self.find(parent);
        let child_root = self.find(child);

        if parent_root == child_root {
            return false;
        }

        self.data[child_root] = parent_root;
        true
    }
}

fn main() {
    let first_line: Vec<usize> = input_user_to_vec();
    let n = first_line[0];
    let m = first_line[1];

    let mut uf = VecUnionFind::new(n);
    let mut cycle_turn = 0;

    for i in 1..=m {
        let edge: Vec<usize> = input_user_to_vec();
        let a = edge[0];
        let b = edge[1];

        if !uf.merge(a, b) && cycle_turn == 0 {
            cycle_turn = i;
        }
    }

    println!("{}", cycle_turn);
}
