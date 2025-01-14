use criterion::{criterion_group, criterion_main, Criterion};
use cpu_benchmark::{run_benchmark, BenchmarkType};

fn benchmark(c: &mut Criterion) {
    c.bench_function("cpu_quick", |b| {
        b.iter(|| run_benchmark(BenchmarkType::Quick))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);