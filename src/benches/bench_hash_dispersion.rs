//! Command to build the index structure

use crate::kmer;
use crate::kmer::Kmer;
use crate::errors::Result;
use crate::hash::fnv_1a_hash::Fnv1aHasher32;
//use crate::hash::kmer_hash::KmerHasher;
//use crate::hash::xxhash::Xxh32Hasher;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BenchHashDispersionArgs {
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
pub fn bench_hash_dispersion(args: BenchHashDispersionArgs) -> Result<()> {
    let hasher: Fnv1aHasher32 = Fnv1aHasher32();

    let mut buckets = vec![0; args.amount_of_buckets];

    for _i in 0 .. args.amount_of_iterations {
        let kmers: Vec<Kmer> = kmer::generate_kmers(args.amount_of_kmers);

        for kmer in kmers {
            let bucket: usize = (hasher.hash(&kmer) % (args.amount_of_buckets as u32)).try_into().unwrap();

            buckets[bucket] += 1;
        }
    }

    for i in 0 .. args.amount_of_buckets {
        println!("{}/{} ({:.1}%)", 
            buckets[i], 
            args.amount_of_kmers * args.amount_of_iterations, 
            buckets[i] as f64 / (args.amount_of_kmers * args.amount_of_iterations) as f64 * 100.0
        );
    }

    Ok(())
}
