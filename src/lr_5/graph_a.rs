use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Copy)]
pub struct Edge {
    pub node: usize,
    pub weight: u32,
}

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    pub fn union(&mut self, node1: usize, node2: usize) {
        let root1: usize = self.find(node1);
        let root2: usize = self.find(node2);

        if root1 != root2 {
            if self.rank[root1] > self.rank[root2] {
                self.parent[root2] = root1;
            } else {
                self.parent[root1] = root2;
                if self.rank[root1] == self.rank[root2] {
                    self.rank[root2] += 1;
                }
            }
        }
    }
}

pub struct Graph {
    adj_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(adj_list: Vec<Vec<Edge>>) -> Self {
        Graph { adj_list }
    }

    pub fn dfs(&self, start: usize, visited: &mut Vec<bool>) {
        visited[start] = true;
        for edge in &self.adj_list[start] {
            if !visited[edge.node] {
                self.dfs(edge.node, visited);
            }
        }
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited: Vec<bool> = vec![false; self.adj_list.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut order: Vec<usize> = Vec::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for edge in &self.adj_list[node] {
                if !visited[edge.node] {
                    visited[edge.node] = true;
                    queue.push_back(edge.node);
                }
            }
        }

        order
    }

    pub fn dijkstra(&self, start: usize) -> Vec<u32> {
        let mut dist: Vec<u32> = vec![u32::MAX; self.adj_list.len()];
        let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if cost > dist[node] {
                continue;
            }

            for edge in &self.adj_list[node] {
                let next = edge.node;
                let next_cost = cost + edge.weight;

                if next_cost < dist[next] {
                    heap.push(Reverse((next_cost, next)));
                    dist[next] = next_cost;
                }
            }
        }

        dist
    }

    pub fn kruskal(&self) -> Vec<Edge> {
        let mut edges: Vec<(usize, usize, u32)> = Vec::new();
        for (node, neighbors) in self.adj_list.iter().enumerate() {
            for edge in neighbors {
                edges.push((node, edge.node, edge.weight));
            }
        }

        edges.sort_by_key(|k| k.2);
        let mut dsu: DisjointSet = DisjointSet::new(self.adj_list.len());
        let mut mst: Vec<Edge> = Vec::new();

        for (u, v, weight) in edges {
            if dsu.find(u) != dsu.find(v) {
                dsu.union(u, v);
                mst.push(Edge { node: v, weight });
            }
        }

        mst
    }

    pub fn prim(&self, start: usize) -> Vec<Edge> {
        let mut mst: Vec<Edge> = Vec::new();
        let mut min_edge: Vec<Option<Edge>> = Vec::with_capacity(self.adj_list.len());
        for _ in 0..self.adj_list.len() {
            min_edge.push(None);
        }
        let mut used: Vec<bool> = vec![false; self.adj_list.len()];
        let mut min_weight: Vec<u32> = vec![u32::MAX; self.adj_list.len()];
        let mut heap: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

        min_weight[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((_weight, node))) = heap.pop() {
            if used[node] {
                continue;
            }

            used[node] = true;
            if let Some(ref edge) = min_edge[node] {
                mst.push(*edge);
            }

            for edge in &self.adj_list[node] {
                if !used[edge.node] && edge.weight < min_weight[edge.node] {
                    min_edge[edge.node] = Some(*edge);
                    min_weight[edge.node] = edge.weight;
                    heap.push(Reverse((edge.weight, edge.node)));
                }
            }
        }

        mst
    }

    pub fn floyd_warshall(&self) -> Vec<Vec<u32>> {
        let n: usize = self.adj_list.len();
        let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; n]; n];

        for i in 0..n {
            dist[i][i] = 0;
        }

        for (i, neighbors) in self.adj_list.iter().enumerate() {
            for edge in neighbors {
                dist[i][edge.node] = edge.weight;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        dist
    }
}
