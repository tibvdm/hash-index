//! All functionality to benchmark hash functions

use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};

/// Benchmark a hash function
fn bench_hash(kmers: &Vec<u64>) {
    // TODO
}

fn bench(c: &mut Criterion) {
    // c.bench_function("bench_hash",
    //     |b| b.iter_with_setup());
}

criterion_group!(benches, bench);
