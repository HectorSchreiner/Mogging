use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mogging::*;

pub fn mogger_benchmark(c: &mut Criterion) {
    c.bench_function("stdout 10k", |b| b.iter(|| println!("Benchmark!")) );
}

criterion_group!(benches, mogger_benchmark);
criterion_main!(benches);
