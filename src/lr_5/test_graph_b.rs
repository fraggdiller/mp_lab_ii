#[cfg(test)]
mod tests {
    use crate::lr_5::{EdgeB as Edge, GraphB as Graph, KosarajuGraph};

    #[test]
    fn test_tarjan_topological_sort() {
        let vertices: usize = 4;
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        adj_list[0].push(Edge { node: 1, weight: 1 });
        adj_list[0].push(Edge { node: 2, weight: 1 });
        adj_list[1].push(Edge { node: 3, weight: 1 });
        adj_list[2].push(Edge { node: 3, weight: 1 });

        let graph: Graph = Graph::new(vertices, adj_list, true);
        let sorted: Vec<usize> = graph.tarjan_topological_sort();
        let correct_orders: [Vec<usize>; 2] = [vec![0, 1, 2, 3], vec![0, 2, 1, 3]];
        assert!(correct_orders.contains(&sorted));
    }

    #[test]
    fn test_tarjan_no_edges() {
        let vertices: usize = 3;
        let adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];

        let graph: Graph = Graph::new(vertices, adj_list, true);
        let sorted: Vec<usize> = graph.tarjan_topological_sort();
        assert_eq!(sorted.len(), vertices);
    }

    #[test]
    fn test_fleury_algorithm() {
        let vertices: usize = 3;
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        adj_list[0].push(Edge { node: 1, weight: 1 });
        adj_list[0].push(Edge { node: 2, weight: 1 });
        adj_list[1].push(Edge { node: 2, weight: 1 });
        adj_list[1].push(Edge { node: 0, weight: 1 });
        adj_list[2].push(Edge { node: 0, weight: 1 });
        adj_list[2].push(Edge { node: 1, weight: 1 });

        let mut graph: Graph = Graph::new(vertices, adj_list, false);
        let eulerian_path: Vec<usize> = graph.fleury_algorithm();
        assert_eq!(eulerian_path, vec![0, 1, 2, 0]);
    }

    #[test]
    fn test_fleury_no_eulerian_path() {
        let vertices: usize = 3;
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        adj_list[0].push(Edge { node: 1, weight: 1 });

        let mut graph: Graph = Graph::new(vertices, adj_list, false);
        let eulerian_path: Vec<usize> = graph.fleury_algorithm();
        assert!(eulerian_path.is_empty());
    }

    #[test]
    fn test_kosaraju_single_sccs() {
        let vertices: usize = 5;
        let mut graph: KosarajuGraph = KosarajuGraph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 0);
        graph.add_edge(4, 2);

        let sccs: Vec<Vec<usize>> = graph.kosaraju();
        assert_eq!(sccs.len(), 1);
        assert!(sccs.contains(&vec![0, 3, 2, 1, 4]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs() {
        let vertices: usize = 8;
        let mut graph: KosarajuGraph = KosarajuGraph::new(vertices);

        graph.add_edge(1, 0);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 7);
        graph.add_edge(4, 7);
        graph.add_edge(6, 4);

        let sccs: Vec<Vec<usize>> = graph.kosaraju();
        assert_eq!(sccs.len(), 4);
        assert!(sccs.contains(&vec![0, 1, 2]));
        assert!(sccs.contains(&vec![3]));
        assert!(sccs.contains(&vec![4, 6, 5]));
        assert!(sccs.contains(&vec![7]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs1() {
        let vertices: usize = 8;
        let mut graph: KosarajuGraph = KosarajuGraph::new(vertices);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 7);
        graph.add_edge(5, 2);
        graph.add_edge(5, 6);
        graph.add_edge(6, 5);
        graph.add_edge(7, 6);

        let sccs: Vec<Vec<usize>> = graph.kosaraju();
        assert_eq!(sccs.len(), 3);
        assert!(sccs.contains(&vec![0]));
        assert!(sccs.contains(&vec![1]));
        assert!(sccs.contains(&vec![2, 5, 6, 7, 4, 3]));
    }

    #[test]
    fn test_kosaraju_no_scc() {
        let vertices: usize = 4;
        let mut graph: KosarajuGraph = KosarajuGraph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let sccs: Vec<Vec<usize>> = graph.kosaraju();
        assert_eq!(sccs.len(), 4);
        for (i, _) in sccs.iter().enumerate().take(vertices) {
            assert_eq!(sccs[i], vec![i]);
        }
    }

    #[test]
    fn test_kosaraju_empty_graph() {
        let vertices: usize = 0;
        let graph: KosarajuGraph = KosarajuGraph::new(vertices);

        let sccs: Vec<Vec<usize>> = graph.kosaraju();
        assert_eq!(sccs.len(), 0);
    }

    #[test]
    fn test_find_eulerian_cycle() {
        let vertices: usize = 3;
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        adj_list[0].push(Edge { node: 1, weight: 1 });
        adj_list[0].push(Edge { node: 2, weight: 1 });
        adj_list[1].push(Edge { node: 2, weight: 1 });
        adj_list[1].push(Edge { node: 0, weight: 1 });
        adj_list[2].push(Edge { node: 0, weight: 1 });
        adj_list[2].push(Edge { node: 1, weight: 1 });

        let mut graph: Graph = Graph::new(vertices, adj_list, false);
        let eulerian_cycle: Vec<usize> = graph.find_eulerian_cycle();
        assert!(eulerian_cycle == vec![0, 1, 2, 0] || eulerian_cycle == vec![0, 2, 1, 0]);
    }

    #[test]
    fn test_find_eulerian_cycle_no_cycle() {
        let vertices: usize = 3;
        let mut adj_list: Vec<Vec<Edge>> = vec![vec![]; vertices];
        adj_list[0].push(Edge { node: 1, weight: 1 });

        let mut graph: Graph = Graph::new(vertices, adj_list, false);
        let eulerian_cycle: Vec<usize> = graph.find_eulerian_cycle();
        assert!(eulerian_cycle.is_empty());
    }
}
