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
pub struct BenchMaxItemsPerBucketSwissprotArgs {
    /// Amount of buckets
    #[structopt(short = "b", long = "buckets", default_value = "10")]
    pub amount_of_buckets: usize,

    /// Conflicts depth to consider
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool
}

/// Implements the bench_hash_conflicts command
pub fn bench_max_items_per_bucket_swissprot(args: BenchMaxItemsPerBucketSwissprotArgs) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(io::stdin());
    
    let hasher: Xxh32Hasher = Xxh32Hasher();

    let mut buckets = vec![0; args.amount_of_buckets];

    let mut kmer_count = 0;
    for record in reader.deserialize() {
        let (kmer_s, _lca, _uids): (String, u32, String) = record?;
        let kmer = Kmer::from(kmer_s);

        let bucket: usize = (hasher.hash(&kmer) % (args.amount_of_buckets as u32)).try_into().unwrap();

        buckets[bucket] += 1;

        kmer_count += 1;
    }

    let mut min: u32 = 999_999_999;
    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for i in 0 .. args.amount_of_buckets {
        if buckets[i] < min {
            min = buckets[i];
        }

        if buckets[i] > max {
            max = buckets[i];
        }

        sum += buckets[i];
    }

    let mean: f64 = sum as f64 / args.amount_of_buckets as f64;

    // println!("#kmers\t#buckets\tmin\tmax\tmean");
    println!("{}\t{}\t{}\t{}\t{}", 
            kmer_count,
            args.amount_of_buckets,
            min, 
            max, 
            mean
        );

    Ok(())
}
