use std::io::{self, stdin};

fn input_user_to_vec() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct App {
    graph: Vec<Vec<usize>>,
    reverse_graph: Vec<Vec<usize>>,
    visited: Vec<bool>,
    order: Vec<usize>,
    component_id: Vec<usize>,
    component_counter: usize,
}

impl App {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            reverse_graph: vec![Vec::new(); n],
            visited: vec![false; n],
            order: Vec::new(),
            component_id: vec![0; n],
            component_counter: 0,
        }
    }

    fn dfs1(&mut self, node: usize) {
        self.visited[node] = true;
        let neighbors = self.graph[node].clone(); 
        for &next in &neighbors {
            if !self.visited[next] {
                self.dfs1(next);
            }
        }
        self.order.push(node);
    }

    fn dfs2(&mut self, node: usize, id: usize) {
        self.visited[node] = true;
        self.component_id[node] = id;
        let neighbors = self.reverse_graph[node].clone();
        for &next in &neighbors {
            if !self.visited[next] {
                self.dfs2(next, id);
            }
        }
    }

    fn run(&mut self, n: usize, m: usize) -> usize {
        for _ in 0..m {
            let row = input_user_to_vec();
            let u = row[0] - 1;
            let v = row[1] - 1;
            self.graph[u].push(v);
            self.reverse_graph[v].push(u); 
        }

        for i in 0..n {
            if !self.visited[i] {
                self.dfs1(i);
            }
        }

        self.visited = vec![false; n];
        self.component_counter = 0;

        while let Some(node) = self.order.pop() {
            if !self.visited[node] {
                self.dfs2(node, self.component_counter);
                self.component_counter += 1;
            }
        }

        let mut scc_in_degree = vec![0; self.component_counter];
        for u in 0..n {
            for &v in &self.graph[u] {
                let cu = self.component_id[u];
                let cv = self.component_id[v];
                if cu != cv {
                    scc_in_degree[cv] += 1;
                }
            }
        }

        let manual_push_count = scc_in_degree.iter().filter(|&&deg| deg == 0).count();

        manual_push_count
    }
}

fn main() {
    let test_cases = input_user_to_vec()[0]; 

    for _ in 0..test_cases {
        let row = input_user_to_vec(); 
        let n = row[0]; 
        let m = row[1]; 

        let mut app = App::new(n); 
        let result = app.run(n, m);
        println!("{}", result);
    }
}
