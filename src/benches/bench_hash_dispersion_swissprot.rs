//! Command to build the index structure

use std::io;

use crate::kmer;
use crate::kmer::Kmer;
use crate::errors::Result;
use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::hash::kmer_hash::KmerHasher;
use crate::hash::xxhash::Xxh32Hasher;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BenchHashDispersionSwissprotArgs {
    /// Amount of k-mers to hash
    #[structopt(short = "i", long = "iterations", default_value = "10")]
    pub amount_of_iterations: usize,

    /// Amount of k-mers to hash
    #[structopt(short = "n", long = "kmers", default_value = "1000")]
    pub amount_of_kmers: usize,

    /// Amount of buckets
    #[structopt(short = "b", long = "buckets", default_value = "10")]
    pub amount_of_buckets: usize
}

/// Implements the bench_hash_dispersion command
pub fn bench_hash_dispersion_swissprot(args: BenchHashDispersionSwissprotArgs) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(io::stdin());

    let hasher: KmerHasher = KmerHasher();

    let mut buckets = vec![0; args.amount_of_buckets];

    let mut aaa = 0;
    for record in reader.deserialize() {
        let (kmer_s, lca, uids): (String, u32, String) = record?;
        let kmer = Kmer::from(kmer_s);

        let bucket: usize = (hasher.hash(&kmer) % (args.amount_of_buckets as u32)).try_into().unwrap();

        buckets[bucket] += 1;

        aaa += 1;
    }

    for i in 0 .. args.amount_of_buckets {
        println!("{}/{} ({:.1}%)", 
            buckets[i], 
            aaa, 
            buckets[i] as f64 / (aaa) as f64 * 100.0
        );
    }

    Ok(())
}
