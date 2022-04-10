//! Implementation of a ConflictTable

use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::kmer::Kmer;
use crate::errors::Result;

// TODO: how about we give the ftable ordening to the hashtable, then this list can be updated in place
// Then later we use this list to write to a csv file in the correct order.

// TODO: Hasher could be static in this case

/// A hash table with uniformly distributed conflicts
pub struct ConflictTable {
    /// The hash function to determine the correct bucket
    hasher: Fnv1aHasher32,
    /// The amount of buckets
    amount_of_buckets: usize,
    /// The buckets
    buckets: Vec<u32>,
    /// The storage for all the conflicts
    stack: Vec<Vec<u64>>
}

impl ConflictTable {
    /// Create a new table with a fixed amount of buckets
    pub fn new(amount_of_buckets: usize) -> ConflictTable {
        ConflictTable {
            hasher: Fnv1aHasher32::new(),
            amount_of_buckets: amount_of_buckets,
            buckets: vec![0; amount_of_buckets],
            stack: vec![]
        }
    }

    /// Insert a kmer in the conflict table
    pub fn insert(&mut self, kmer: &Kmer) {
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
    }

    /// Get the value for a k-mer
    pub fn get(&self, kmer: &Kmer) -> Result<u32> {
        // Calculate the bucket of this k-mer
        let bucket = (self.hasher.hash(kmer) % self.amount_of_buckets as u32) as usize;

        // Retrieve the position of the first conflict
        let first_conflict_index: usize = self.buckets[bucket] as usize;

        // Perform a logarithmic search to find the correct entry
        self.log_search(kmer, first_conflict_index - 1)
    }

    /// Check if a k-mer is contained in the index
    pub fn contains(&self, kmer: &Kmer) -> bool {
        match self.get(kmer) {
            Ok(_)  => true,
            Err(_) => false
        }
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

    /// Logarithmic search for conflicts
    fn log_search(&self, kmer: &Kmer, first_conflict_index: usize) -> Result<u32> {
        let mut lower: i32 = 0;
        let mut upper: i32 = self.stack[first_conflict_index].len() as i32 - 1;

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
