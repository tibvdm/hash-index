use error_chain::quick_main;

use structopt::StructOpt;

use hash_index::benches;
use hash_index::commands;
use hash_index::errors::Result;

quick_main!(|| -> Result<()> {
    match Opt::from_args() {
        Opt::BuildIndex(args) => commands::buildindex::buildindex(args),

        Opt::BenchHashDispersion(args) => benches::bench_hash_dispersion::bench_hash_dispersion(args),
        Opt::BenchHashDispersionSwissprot(args) => benches::bench_hash_dispersion_swissprot::bench_hash_dispersion_swissprot(args),
        Opt::BenchHashConflicts(args) => benches::bench_hash_conflicts::bench_hash_conflicts(args),
        Opt::BenchMaxItemsPerBucket(args) => benches::bench_max_items_per_bucket::bench_max_items_per_bucket(args),
        Opt::BenchMaxItemsPerBucketSwissprot(args) => benches::bench_max_items_per_bucket_swissprot::bench_max_items_per_bucket_swissprot(args)
    }
});

#[rustfmt::skip]
#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "buildindex")] BuildIndex(commands::buildindex::BuildIndexArgs),

    #[structopt(name = "bench_hash_dispersion")] BenchHashDispersion(benches::bench_hash_dispersion::BenchHashDispersionArgs),
    #[structopt(name = "bench_hash_dispersion_swissprot")] BenchHashDispersionSwissprot(benches::bench_hash_dispersion_swissprot::BenchHashDispersionSwissprotArgs),
    #[structopt(name = "bench_hash_conflicts")] BenchHashConflicts(benches::bench_hash_conflicts::BenchHashConflictsArgs),
    #[structopt(name = "bench_max_items_per_bucket")] BenchMaxItemsPerBucket(benches::bench_max_items_per_bucket::BenchMaxItemsPerBucketArgs),
    #[structopt(name = "bench_max_items_per_bucket_swissprot")] BenchMaxItemsPerBucketSwissprot(benches::bench_max_items_per_bucket_swissprot::BenchMaxItemsPerBucketSwissprotArgs)
}
