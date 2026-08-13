use criterion::{criterion_group, criterion_main, Criterion};
use DSAR_RantAI::ch_01::graph::UndirectedGraph;
use std::hint::black_box;

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn setup(edges: usize) -> UndirectedGraph {
    let mut graph = UndirectedGraph::new();

    for vertex in ALPHABET {
        graph.add_vertex(vertex);
    }

    let max_edges = ALPHABET.len() * (ALPHABET.len() - 1) / 2;
    assert!(
        edges <= max_edges,
        "an alphabet graph supports at most {max_edges} unique undirected edges"
    );

    let mut added_edges = 0;
    for (index, &from) in ALPHABET.iter().enumerate() {
        for &to in &ALPHABET[index + 1..] {
            if added_edges == edges {
                return graph;
            }
            graph.add_edge((from, to));
            added_edges += 1;
        }
    }

    graph
}

fn benchmark_graph(c: &mut Criterion, edges: usize) {
    let graph = setup(edges);

    c.bench_function(&format!("graph.dfs/{} edges", edges), |b| {
        b.iter(|| black_box(graph.dfs('A')))
    });
    c.bench_function(&format!("graph.bfs/{} edges", edges), |b| {
        b.iter(|| black_box(graph.bfs('A')))
    });
}

fn graph_benchmarks(c: &mut Criterion) {
    for edges in [100, 200, 300, 325] {
        benchmark_graph(c, edges);
    }
}

criterion_group!(benches, graph_benchmarks);
criterion_main!(benches);
