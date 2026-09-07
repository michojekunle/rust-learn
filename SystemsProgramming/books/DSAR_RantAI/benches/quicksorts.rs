use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use DSAR_RantAI::ch_01::{parallel_quicksort::parallel_quicksort, quicksort::quick_sort};

const SIZES: [usize; 5] = [100, 1_000, 10_000, 100_000, 1_000_000];

/// Builds deterministic, deliberately unsorted input without adding random-number
/// generation overhead or an extra benchmark dependency.
fn setup(size: usize) -> Vec<usize> {
    (0..size)
        .map(|index| (index.wrapping_mul(37).wrapping_add(11)) % size)
        .collect()
}

fn benchmark_quicksorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("quicksort");

    for size in SIZES {
        let input = setup(size);

        group.bench_with_input(BenchmarkId::new("sequential", size), &input, |b, input| {
            b.iter(|| quick_sort(black_box(input.clone())))
        });

        group.bench_with_input(BenchmarkId::new("parallel", size), &input, |b, input| {
            b.iter(|| parallel_quicksort(black_box(input.clone())))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_quicksorts);
criterion_main!(benches);
