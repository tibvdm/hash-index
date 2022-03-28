use error_chain::quick_main;

use structopt::StructOpt;

use hash_index::benches;
use hash_index::commands;
use hash_index::errors::Result;

quick_main!(|| -> Result<()> {
    match Opt::from_args() {
        Opt::BuildIndex(args) => commands::buildindex::buildindex(args),

        Opt::BenchHashDispersion(args) => benches::bench_hash_dispersion::bench_hash_dispersion(args),
        Opt::BenchHashConflicts(args) => benches::bench_hash_conflicts::bench_hash_conflicts(args)
    }
});

#[rustfmt::skip]
#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "buildindex")] BuildIndex(commands::buildindex::BuildIndexArgs),

    #[structopt(name = "bench_hash_dispersion")] BenchHashDispersion(benches::bench_hash_dispersion::BenchHashDispersionArgs),
    #[structopt(name = "bench_hash_conflicts")] BenchHashConflicts(benches::bench_hash_conflicts::BenchHashConflictsArgs)
}
