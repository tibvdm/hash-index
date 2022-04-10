//! TODO: First hashing that distributes the k-mers to less buckets

use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::kmer::Kmer;
use crate::errors::Result;

// TODO: how about we give the ftable ordening to the hashtable, then this list can be updated in place
// Then later we use this list to write to a csv file in the correct order.

// TODO: no need to update bucket counters, because 2D vector
// !!! However, bucket indices have to point to 1D indices, and later fix this in finish();

// TODO: Hasher could be static in this case

// TODO: no need to always update the counts, because we work with 2D arrays

/// TODO
pub struct HashTable {
    /// TODO
    hasher: Fnv1aHasher32,
    /// TODO
    amount_of_buckets: usize,
    /// TODO
    buckets: Vec<u32>,
    /// TODO
    stack: Vec<Vec<u64>>
}

impl HashTable {
    /// TODO
    pub fn new(amount_of_buckets: usize) -> HashTable {
        HashTable {
            hasher: Fnv1aHasher32::new(),
            amount_of_buckets: amount_of_buckets,
            buckets: vec![0; amount_of_buckets],
            stack: vec![]
        }
    }

    /// TODO
    pub fn insert(&mut self, kmer: &Kmer) {
        // TODO: keep track of order of kmers in secondary structure
        // Then we can reorder the function table to represent this order
        // Then fp = bucket_index + offset_first_conflict

        let bucket: usize = (self.hasher.hash(kmer) % self.amount_of_buckets as u32) as usize;

        // The bucket contains the position of the first conflict
        let mut first_conflict_index: usize = self.buckets[bucket] as usize;

        // This is the first hit for our bucket
        if first_conflict_index == 0 {
            // Prepare the entry to be pushed.
            let entry: u64 = kmer.0;

            // add the entry to the list
            self.stack.push(vec![entry]);

            // Store the index of this conflict in the bucket
            self.buckets[bucket] = self.stack.len() as u32;
        }

        // This is a conflict for our bucket
        else {
            // Because 0 represents an empty bucket
            // 1 represents the first bucket. So 1 -> 0, 2 -> 1, ...
            first_conflict_index -= 1;

            // prepare entry to be pushed
            let entry: u64 = kmer.0;

            // Add the new conflict to the list
            self.stack[first_conflict_index].push(entry);
        }

        println!("{:?}", self.buckets);
        println!("{:?}", self.stack);
    }

    /// TODO
    pub fn get(&self, kmer: &Kmer) -> Result<u32> {
        // Calculate the bucket of this k-mer
        let bucket = (self.hasher.hash(kmer) % self.amount_of_buckets as u32) as usize;

        // Retrieve the position of the first conflict
        let first_conflict_index: usize = self.buckets[bucket] as usize;

        // Perform a logarithmic search to find the correct entry
        self.log_search(kmer, first_conflict_index - 1)
    }

    /// TODO
    pub fn finish() {
        // TODO: order is OK, because we start with an ordered list of kmers ()
        // TODO: place the 2D vector sequentieel in memory
        // TODO: maybe final changes voor de ftable order list

        // TODO: set the amount of conlicts in bucket bits
        // Set #conflicts to 1 (= 0x0000200000000000)
        // Clear #conflicts (= 0x00001FFFFFFFFFFF)
        // Shift #conflicst to 19 bits (amount_of_conflicts << 45)
    }

    fn log_search(&self, kmer: &Kmer, first_conflict_index: usize) -> Result<u32> {
        let mut lower: i32 = 0;
        let mut upper: i32 = self.stack[first_conflict_index].len() as i32 - 1;

        println!("{}", lower);
        println!("{}", upper);

        println!("{:?}", self.stack[first_conflict_index]);
        println!("{:?}", kmer);

        while lower <= upper {
            let middle: i32 = (lower + upper) / 2;
            if self.stack[first_conflict_index][middle as usize] < kmer.0 {
                lower = middle + 1;
            } else if self.stack[first_conflict_index][middle as usize] > kmer.0 {
                upper = middle - 1;
            } else {
                return Ok((first_conflict_index as i32 + middle) as u32);
            }
        }

        bail!("Entry not found")
    }
}

// error_chain! {
//     errors {
//         EntryNotFound() {
//             description("k-mer could not be found!"), // note the ,
//             display("k-mer could not be found!"), // trailing comma is allowed
//         }
//     }
// }
