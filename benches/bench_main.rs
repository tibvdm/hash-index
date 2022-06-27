use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    //benchmarks::bench_fnv_hash::benches,
    //benchmarks::bench_fnv_optimized_hash::benches,
    //benchmarks::bench_xxhash::benches,
    //benchmarks::bench_kmer_hash::benches,

    benchmarks::bench_conflict_table::benches,
    //benchmarks::bench_conflict_lca::benches,
    benchmarks::bench_fst::benches,
}
