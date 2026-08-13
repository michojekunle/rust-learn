use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, PartialEq, Debug)]
pub struct UndirectedGraph {
    list: HashMap<char, Vec<char>>,
}

impl UndirectedGraph {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, u: char) {
        self.list.entry(u).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, (u, v): (char, char)) {
        self._add_edge_x((u, v));
        self._add_edge_x((v, u));
    }

    fn _add_edge_x(&mut self, (x, y): (char, char)) {
        let vertex_edges = self.list.entry(x).or_insert_with(Vec::new);

        if !vertex_edges.contains(&y) {
            vertex_edges.push(y);
        }
    }

    fn remove_vertex(&mut self, u: char) {
        if let Some(neighbours) = self.list.remove(&u) {
            for v in neighbours {
                self._remove_edge_x((v, u));
            }
        }
    }

    fn remove_edge(&mut self, (u, v): (char, char)) {
        self._remove_edge_x((u, v));
        self._remove_edge_x((v, u));
    }

    fn _remove_edge_x(&mut self, (x, y): (char, char)) {
        if let Some(vertex_edges) = self.list.get_mut(&x) {
            vertex_edges.retain(|&e| e != y);
        }
    }

    pub fn has_edge(&self, (u, v): (char, char)) -> bool {
        match self.list.get(&u) {
            None => false,
            Some(edges) => edges.contains(&v),
        }
    }

    pub fn dfs(&self, u: char) -> Vec<char> {
        let mut agenda: Vec<char> = Vec::new();
        agenda.push(u);

        let mut visited: HashSet<char> = HashSet::new();
        let mut result: Vec<char> = Vec::new();

        while !agenda.is_empty() {
            let curr_v: char = agenda.pop().unwrap();

            if !visited.contains(&curr_v) {
                result.push(curr_v);
                visited.insert(curr_v);

                if let Some(vertex_edges) = self.list.get(&curr_v) {
                    for v in vertex_edges.iter().rev() {
                        if !visited.contains(v) {
                            agenda.push(*v);
                        }
                    }
                }
            }
        }
        result
    }

    pub fn bfs(&self, u: char) -> Vec<char> {
        let mut agenda: VecDeque<char> = VecDeque::new();
        agenda.push_back(u);

        let mut visited: HashSet<char> = HashSet::new();
        visited.insert(u);

        let mut result: Vec<char> = Vec::new();

        while !agenda.is_empty() {
            let curr_v: char = agenda.pop_front().unwrap();
            result.push(curr_v);

            if let Some(vertex_edges) = self.list.get(&curr_v) {
                for v in vertex_edges {
                    if !visited.contains(v) {
                        visited.insert(*v);
                        agenda.push_back(*v);
                    }
                }
            }
        }
        result
    }

    fn print(&self) {
        for (vertex, edges) in &self.list {
            print!("{} -> {:?}", vertex, edges);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut graph = UndirectedGraph::new();

        graph.print();

        // Test adding edges
        graph.add_edge(('A', 'B'));
        graph.add_edge(('B', 'C'));

        // Verify A connects to B, and B connects to A (undirected)
        assert!(graph.list.get(&'A').unwrap().contains(&'B'));
        assert!(graph.list.get(&'B').unwrap().contains(&'A'));

        // Test duplicate prevention
        graph.add_edge(('A', 'B'));
        let a_edges = graph.list.get(&'A').unwrap();
        assert_eq!(a_edges.iter().filter(|&&e| e == 'B').count(), 1);

        // Test removing edges
        graph.remove_edge(('A', 'B'));
        assert!(!graph.list.get(&'A').unwrap().contains(&'B'));
        assert!(!graph.list.get(&'B').unwrap().contains(&'A'));
    }

    #[test]
    fn test_has_edge() {
        let mut graph = UndirectedGraph::new();

        // Setup initial graph structure
        graph.add_edge(('A', 'B'));
        graph.add_edge(('B', 'C'));

        // Test active connections
        assert!(graph.has_edge(('A', 'B')));
        assert!(graph.has_edge(('B', 'A'))); // Must be true both ways
        assert!(graph.has_edge(('B', 'C')));

        // Test non-existent connections
        assert!(!graph.has_edge(('A', 'C'))); // Indirectly connected, but no direct edge
        assert!(!graph.has_edge(('A', 'Z'))); // Target vertex does not exist in graph
        assert!(!graph.has_edge(('Z', 'Y'))); // Neither vertex exists
    }

    #[test]
    fn test_remove_vertex() {
        let mut graph = UndirectedGraph::new();

        // Setup a small network: A connects to B and C
        graph.add_edge(('A', 'B'));
        graph.add_edge(('A', 'C'));

        // Verify edges exist initially
        assert!(graph.has_edge(('A', 'B')));
        assert!(graph.has_edge(('A', 'C')));

        // Remove vertex 'A'
        graph.remove_vertex('A');

        // 1. Verify 'A' itself is completely gone from the graph keys
        assert!(!graph.list.contains_key(&'A'));

        // 2. Verify references to 'A' are wiped out from its old neighbors
        if let Some(b_edges) = graph.list.get(&'B') {
            assert!(!b_edges.contains(&'A'));
        }
        if let Some(c_edges) = graph.list.get(&'C') {
            assert!(!c_edges.contains(&'A'));
        }

        // 3. Verify 'B' and 'C' still exist in the map as independent vertices
        assert!(graph.list.contains_key(&'B'));
        assert!(graph.list.contains_key(&'C'));
    }

    #[test]
    fn test_bfs() {
        let mut graph = UndirectedGraph::new();
        // Setup a simple tree-like structure:
        //     A
        //    / \
        //   B   C
        //  /
        // D
        graph.add_edge(('A', 'B'));
        graph.add_edge(('A', 'C'));
        graph.add_edge(('B', 'D'));

        // BFS explores level-by-level
        let bfs_result = graph.bfs('A');
        assert_eq!(bfs_result, vec!['A', 'B', 'C', 'D']);
    }

    #[test]
    fn test_dfs() {
        let mut graph = UndirectedGraph::new();
        // Setup same structure
        graph.add_edge(('A', 'B'));
        graph.add_edge(('A', 'C'));
        graph.add_edge(('B', 'D'));

        // DFS explores deeply down one path first (A -> B -> D) before backtracking to C
        let dfs_result = graph.dfs('A');
        assert_eq!(dfs_result, vec!['A', 'B', 'D', 'C']);
    }
}
