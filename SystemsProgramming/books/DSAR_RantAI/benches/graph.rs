use criterion::{criterion_group, criterion_main, Criterion};
use DSAR_RantAI::ch_01::graph::UndirectedGraph;
use std::hint::black_box;

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Clone, Copy)]
enum Density {
    Sparse,
    Medium,
    Dense,
}

impl Density {
    fn name(self) -> &'static str {
        match self {
            Density::Sparse => "sparse",
            Density::Medium => "medium",
            Density::Dense => "dense",
        }
    }
}

fn setup(vertices: usize, density: Density) -> UndirectedGraph {
    let mut graph = UndirectedGraph::new();

    for vertex in 0..vertices {
        graph.add_vertex(char::from_u32(vertex as u32).unwrap());
    }

    let max_edges = vertices * (vertices -1) / 2;

    let target_edges = match density {
        Density::Sparse => vertices.saturating_sub(1),
        Density::Medium => (2 * vertices).min(max_edges),
        Density::Dense => max_edges / 4
    };

    for vertex in 1..=vertices {
        graph.add_edge((char::from_u32((vertex - 1) as u32).unwrap(), char::from_u32(vertex as u32).unwrap()));
    }

    let mut edges_added = vertices.saturating_sub(1);

    'outer: for from in 0..vertices {
        for to in (from + 1)..vertices {
            if edges_added >= target_edges {
                break 'outer;
            }

            if to == from + 1 {
                continue
            }

            graph.add_edge((char::from_u32(from as u32).unwrap(), char::from_u32(to as u32).unwrap()));
            edges_added += 1;
        }
    }

    graph
}

fn benchmark_graph(c: &mut Criterion, vertices: usize, density: Density) {
    let graph = setup(vertices, density);
    let density_name = density.name();

    c.bench_function(&format!("graph/{density_name}/dfs/{vertices} vertices"), |b| {
        b.iter(|| black_box(graph.dfs('0')))
    });
    c.bench_function(&format!("graph/{density_name}/bfs/{vertices} vertices"), |b| {
        b.iter(|| black_box(graph.bfs('0')))
    });
}

fn graph_benchmarks(c: &mut Criterion) {
    for vertices in [10, 100, 200, 300, 500, 1000] {
        benchmark_graph(c, vertices, Density::Sparse);
        benchmark_graph(c, vertices, Density::Medium);
        benchmark_graph(c, vertices, Density::Dense);
    }
}

criterion_group!(benches, graph_benchmarks);
criterion_main!(benches);
