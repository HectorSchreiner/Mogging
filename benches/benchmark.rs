use criterion::{criterion_group, criterion_main, Criterion};
use mogging::*;

pub fn mogger_benchmark(c: &mut Criterion) {
    Mogger::create_default_mogger().init();
    c.bench_function("mogger debug macro", |b| b.iter(|| test!("benchmark")));
}

criterion_group!(benches, mogger_benchmark);
criterion_main!(benches);
