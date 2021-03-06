//! Command to build the index structure

use crate::kmer;
use crate::kmer::Kmer;
use crate::errors::Result;
use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::hash::kmer_hash::KmerHasher;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BenchHashConflictsArgs {
    /// Amount of k-mers to hash
    #[structopt(short = "i", long = "iterations", default_value = "1")]
    pub amount_of_iterations: usize,

    /// Amount of k-mers to hash
    #[structopt(short = "n", long = "kmers", default_value = "1000")]
    pub amount_of_kmers: usize,

    /// Amount of buckets
    #[structopt(short = "b", long = "buckets", default_value = "10")]
    pub amount_of_buckets: usize,

    /// Conflicts depth to consider
    #[structopt(short = "d", long = "depth", default_value = "16")]
    pub conflict_depth: usize,

    /// Conflicts depth to consider
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool
}

/// Implements the bench_hash_conflicts command
pub fn bench_hash_conflicts(args: BenchHashConflictsArgs) -> Result<()> {
    let hasher: KmerHasher = KmerHasher();

    let mut buckets = vec![0; args.amount_of_buckets];

    let mut conflicts = vec![0; args.conflict_depth];

    for i in 0 .. args.amount_of_iterations {
        if args.verbose && (i % (args.amount_of_iterations / 10) == 0) {
            eprintln!("{}", i);
        }

        let kmers: Vec<Kmer> = kmer::generate_kmers(args.amount_of_kmers);

        for kmer in kmers {
            let bucket: usize = (hasher.hash(&kmer) % (args.amount_of_buckets as u32)).try_into().unwrap();

            buckets[bucket] += 1;

            for i in 0 .. args.conflict_depth {
                if buckets[bucket] > i + 1 {
                    conflicts[i as usize] += 1;
                }
            }
        }

        for i in 0 .. args.amount_of_buckets {
            buckets[i] = 0;
        }
    }

    for i in 0 .. args.conflict_depth {
        println!("conflicts (i: {}): {}/{}", 
            i, 
            conflicts[i] as f64 / args.amount_of_iterations as f64, 
            args.amount_of_kmers
        );
    }

    Ok(())
}
