#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[path = "../src/day11.rs"]
mod day11;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 11", |b| b.iter(|| day11::solve_part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
