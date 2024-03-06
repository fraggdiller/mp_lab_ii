#[derive(Clone, Copy)]
pub struct Edge {
    pub node: usize,
    pub weight: u32,
}

#[derive(Clone)]
pub struct Graph {
    vertices: usize,
    pub adj_list: Vec<Vec<Edge>>,
    pub transpose_adj_list: Vec<Vec<Edge>>,
    is_directed: bool,
}

pub struct KosarajuGraph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
    transpose_adj_list: Vec<Vec<usize>>,
}

impl KosarajuGraph {
    pub fn new(vertices: usize) -> Self {
        KosarajuGraph {
            vertices,
            adj_list: vec![vec![]; vertices],
            transpose_adj_list: vec![vec![]; vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.transpose_adj_list[v].push(u);
    }

    pub fn dfs(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
        visited[node] = true;
        for &neighbor in &self.adj_list[node] {
            if !visited[neighbor] {
                self.dfs(neighbor, visited, stack);
            }
        }
        stack.push(node);
    }

    pub fn dfs_scc(&self, node: usize, visited: &mut Vec<bool>, scc: &mut Vec<usize>) {
        visited[node] = true;
        scc.push(node);
        for &neighbor in &self.transpose_adj_list[node] {
            if !visited[neighbor] {
                self.dfs_scc(neighbor, visited, scc);
            }
        }
    }

    pub fn kosaraju(&self) -> Vec<Vec<usize>> {
        let mut visited: Vec<bool> = vec![false; self.vertices];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..self.vertices {
            if !visited[i] {
                self.dfs(i, &mut visited, &mut stack);
            }
        }

        let mut sccs: Vec<Vec<usize>> = Vec::new();
        visited = vec![false; self.vertices];

        while let Some(node) = stack.pop() {
            if !visited[node] {
                let mut scc = Vec::new();
                self.dfs_scc(node, &mut visited, &mut scc);
                sccs.push(scc);
            }
        }

        sccs
    }
}


impl Graph {
    pub fn new(vertices: usize, adj_list: Vec<Vec<Edge>>, is_directed: bool) -> Self {
        let mut transpose_adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        for (u, edges) in adj_list.iter().enumerate() {
            for edge in edges {
                transpose_adj_list[edge.node].push(Edge { node: u, weight: edge.weight });
            }
        }

        Graph {
            vertices,
            adj_list,
            transpose_adj_list,
            is_directed,
        }
    }

    fn dfs_topological(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
        visited[node] = true;
        for edge in &self.adj_list[node] {
            if !visited[edge.node] {
                self.dfs_topological(edge.node, visited, stack);
            }
        }
        stack.push(node);
    }

    pub fn tarjan_topological_sort(&self) -> Vec<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.adj_list.len()];
        let mut result: Vec<usize> = Vec::new();

        for node in 0..self.adj_list.len() {
            if !visited[node] {
                self.dfs_topological(node, &mut visited, &mut stack);
            }
        }

        while let Some(node) = stack.pop() {
            result.push(node);
        }

        result
    }

    fn is_eulerian(&self) -> bool {
        let mut odd_degree_nodes: i32 = 0;
        for adj in &self.adj_list {
            let degree: usize = adj.len();
            if degree % 2 != 0 {
                odd_degree_nodes += 1;
            }
        }
        odd_degree_nodes == 0 || odd_degree_nodes == 2
    }

    fn remove_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].retain(|edge| edge.node != v);
        if !self.is_directed {
            self.adj_list[v].retain(|edge| edge.node != u);
        }
    }

    pub fn fleury_algorithm(&mut self) -> Vec<usize> {
        if !self.is_eulerian() {
            return vec![];
        }

        let mut path: Vec<usize> = Vec::new();
        let mut u: usize = self.adj_list.iter().position(|adj| !adj.is_empty()).unwrap_or(0);

        path.push(u);
        while let Some(&Edge { node: v, .. }) = self.adj_list[u].first() {
            path.push(v);
            self.remove_edge(u, v);
            u = v;
        }

        path
    }

    pub fn find_eulerian_cycle(&mut self) -> Vec<usize> {
        if !self.is_eulerian() {
            return vec![];
        }

        let mut cycle: Vec<usize> = Vec::new();
        let mut current_path: Vec<usize> = Vec::new();
        let mut u: usize = 0;

        current_path.push(u);
        while !current_path.is_empty() {
            if !self.adj_list[u].is_empty() {
                current_path.push(u);
                let v: usize = self.adj_list[u][0].node;
                self.remove_edge(u, v);
                u = v;
            } else {
                cycle.push(u);
                u = current_path.pop().unwrap();
            }
        }

        cycle
    }
}