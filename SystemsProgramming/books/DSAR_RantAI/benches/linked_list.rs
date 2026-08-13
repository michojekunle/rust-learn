use criterion::{criterion_group, criterion_main, Criterion};
use DSAR_RantAI::ch_01::linked_list::List;
use std::hint::black_box;

fn setup(size: usize) -> List {
    let mut list = List::new();

    for i in 1..=size {
        list.push(i as i32);
    }

    list
}

fn benchmark_linked_list(c: &mut Criterion, size: usize) {
    let list = setup(size);

    c.bench_function(&format!("LinkedList Search missing/{} size", size), |b| {
        b.iter(|| black_box(list.search(-1)));
    });

      c.bench_function(&format!("LinkedList Search middle/{} size", size), |b| {
        b.iter(|| black_box(list.search((size/2 ) as i32)));
    });

      c.bench_function(&format!("LinkedList Search near head/{} size", size), |b| {
        b.iter(|| black_box(list.search((size - 1) as i32)));
    });

      c.bench_function(&format!("LinkedList Search near tail/{} size", size), |b| {
        b.iter(|| black_box(list.search(0)));
    });
}

fn linked_list_benchmarks(c: &mut Criterion) {
    for size in [100, 500, 1000, 10000, 100000] {
        benchmark_linked_list(c, size);
    }
}

criterion_group!(benches, linked_list_benchmarks);
criterion_main!(benches);
