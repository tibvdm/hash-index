//! All functionality to benchmark hash functions

use std::fs;

use criterion::{criterion_group, Criterion};
use core::time::Duration;

use hash_index::index::conflict_table::ConflictTable;
use hash_index::index::lca_table::LcaTable;
use hash_index::kmer::Kmer;
use hash_index::kmer;

/// Benchmark a hash function
fn bench_fst(fst: &fst::Map, kmers: &Vec<String>) {
    for kmer in kmers {
        fst.get(kmer);
    }
}

/// TODO
fn bench(c: &mut Criterion) {
    let bytes = fs::read("results/index/index.fst".to_string()).expect("Whatever");
    let fst = fst::Map::from_bytes(bytes).expect("Whatever");

    let kmers: Vec<String> = kmer::generate_kmers(1_000_000)
        .into_iter()
        .map(|k| String::from(k))
        .collect();

    c.bench_function("bench_fst",
        |b| b.iter_with_setup(|| kmers.to_vec(), |kmers| bench_fst(&fst, &kmers))
    );
}

fn custom_criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(300))
        .sample_size(1000)
}

criterion_group!(
    name = benches;
    config = custom_criterion_config();
    targets = bench
);
