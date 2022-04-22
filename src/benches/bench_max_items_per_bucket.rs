//! Command to build the index structure

use crate::kmer;
use crate::kmer::Kmer;
use crate::errors::Result;
use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::hash::kmer_hash::KmerHasher;
use crate::hash::xxhash::Xxh32Hasher;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BenchMaxItemsPerBucketArgs {
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
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool
}

/// Implements the bench_hash_conflicts command
pub fn bench_max_items_per_bucket(args: BenchMaxItemsPerBucketArgs) -> Result<()> {
    let hasher: Xxh32Hasher = Xxh32Hasher();

    let mut buckets = vec![0; args.amount_of_buckets];

    // let mut conflicts = vec![0; args.conflict_depth];

    let mut maxes: Vec<u32> = vec![0; args.amount_of_iterations];

    for i in 0 .. args.amount_of_iterations {
        if args.verbose && (i % (args.amount_of_iterations / 10) == 0) {
            eprintln!("{}", i);
        }

        let kmers: Vec<Kmer> = kmer::generate_kmers(args.amount_of_kmers);

        for kmer in kmers {
            let bucket: usize = (hasher.hash(&kmer) % (args.amount_of_buckets as u32)).try_into().unwrap();

            buckets[bucket] += 1;
        }

        let mut max: u32 = 0;
        for i in 0 .. args.amount_of_buckets {
            if buckets[i] > max {
                max = buckets[i];
            }

            buckets[i] = 0;
        }

        maxes[i] = max;
    }

    let mut min: u32 = 999_999_999;
    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for i in 0 .. args.amount_of_iterations {
        if maxes[i] < min {
            min = maxes[i];
        }

        if maxes[i] > max {
            max = maxes[i];
        }

        sum += maxes[i];
    }

    let mean: f64 = sum as f64 / args.amount_of_iterations as f64;

    let mut var: f64 = 0.0;
    for i in 0 .. args.amount_of_iterations {
        var += (maxes[i] as f64 - mean).powf(2.0);
    }

    let sd = (var / args.amount_of_iterations as f64).sqrt();

    // println!("#iterations\t#kmers\t#buckets\tmin\tmax\tmean\tsd");
    println!("{}\t{}\t{}\t{}\t{}\t{}\t{}", 
            args.amount_of_iterations,
            args.amount_of_kmers,
            args.amount_of_buckets,
            min, 
            max, 
            mean,
            sd
        );

    Ok(())
}
