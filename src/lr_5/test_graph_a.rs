#[cfg(test)]
mod tests {
    use crate::lr_5::{Edge, Graph};

    fn create_test_graph() -> Graph {
        let adj_list: Vec<Vec<Edge>> = vec![
            vec![Edge { node: 1, weight: 2 }, Edge { node: 2, weight: 4 }],
            vec![Edge { node: 2, weight: 1 }, Edge { node: 3, weight: 7 }],
            vec![Edge { node: 0, weight: 4 }, Edge { node: 3, weight: 3 }],
            vec![],
        ];

        Graph::new(adj_list)
    }

    #[test]
    fn test_dfs() {
        let graph: Graph = create_test_graph();
        let mut visited: Vec<bool> = vec![false; 4];
        graph.dfs(0, &mut visited);
        assert_eq!(visited, vec![true, true, true, true]);
    }

    #[test]
    fn test_bfs() {
        let graph: Graph = create_test_graph();
        let order: Vec<usize> = graph.bfs(0);
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dijkstra() {
        let graph: Graph = create_test_graph();
        let dist: Vec<u32> = graph.dijkstra(0);
        assert_eq!(dist, vec![0, 2, 3, 6]);
    }

    #[test]
    fn test_kruskal() {
        let graph: Graph = create_test_graph();
        let mst: Vec<Edge> = graph.kruskal();
        let mst_weights: Vec<u32> = mst.iter().map(|e| e.weight).collect();
        assert_eq!(mst_weights, vec![1, 2, 3]);
    }

    #[test]
    fn test_prim() {
        let graph: Graph = create_test_graph();
        let mst: Vec<Edge> = graph.prim(0);
        let mst_weights: Vec<u32> = mst.iter().map(|e| e.weight).collect();
        assert_eq!(mst_weights, vec![2, 1, 3]);
    }

    #[test]
    fn test_floyd_warshall() {
        let graph: Graph = create_test_graph();
        let dist: Vec<Vec<u32>> = graph.floyd_warshall();
        let expected: Vec<Vec<u32>> = vec![
            vec![0, 2, 3, 6],
            vec![5, 0, 1, 4],
            vec![4, 6, 0, 3],
            vec![u32::MAX, u32::MAX, u32::MAX, 0],
        ];
        assert_eq!(dist, expected);
    }
}
