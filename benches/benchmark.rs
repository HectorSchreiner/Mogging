use criterion::{criterion_group, criterion_main, Criterion};
use mogging::*;

pub fn mogger_benchmark(c: &mut Criterion) {
    Mogger::default();
    //c.bench_function("mogger debug macro", |b| b.iter(|| ));
}

criterion_group!(benches, mogger_benchmark);
criterion_main!(benches);
