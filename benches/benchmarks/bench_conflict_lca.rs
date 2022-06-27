//! All functionality to benchmark hash functions

use criterion::{criterion_group, Criterion};
use core::time::Duration;

use hash_index::index::conflict_table::ConflictTable;
use hash_index::index::lca_table::LcaTable;
use hash_index::kmer::Kmer;
use hash_index::kmer;

/// Benchmark a hash function
fn bench_conflict_lca(conflict_table: &ConflictTable, lca_table: &LcaTable, kmers: &Vec<Kmer>) {
    for kmer in kmers {
        match conflict_table.get(kmer) {
            Some(id) => lca_table.get(id as usize),
            None => 0
        };
    }
}

/// TODO
fn bench(c: &mut Criterion) {
    let conflict_table: ConflictTable = ConflictTable::from_bin("results/hash.bin".to_string());
    let lca_table: LcaTable = LcaTable::from_bin("results/lca.bin".to_string());
    let kmers: Vec<Kmer> = kmer::generate_kmers(1_000_000);

    c.bench_function("bench_conflict_lca",
        |b| b.iter_with_setup(|| kmers.to_vec(), |kmers| bench_conflict_lca(&conflict_table, &lca_table, &kmers))
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
