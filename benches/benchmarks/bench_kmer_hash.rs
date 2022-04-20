//! All functionality to benchmark hash functions

use criterion::{criterion_group, Criterion};
use core::time::Duration;

use hash_index::hash::kmer_hash::KmerHasher;
use hash_index::kmer::Kmer;
use hash_index::kmer;

/// Benchmark a hash function
fn bench_hash(kmers: &Vec<Kmer>) {
    let hasher = KmerHasher();

    for kmer in kmers {
        hasher.hash(kmer);
    }
}

/// TODO
fn bench(c: &mut Criterion) {
    let kmers: Vec<Kmer> = kmer::generate_kmers(1_000_000);

    c.bench_function("bench_kmer_hash",
        |b| b.iter_with_setup(|| kmers.to_vec(), |kmers| bench_hash(&kmers))
    );
}

fn custom_criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(60))
        .sample_size(1_000)
}

criterion_group!(
    name = benches;
    config = custom_criterion_config();
    targets = bench
);
