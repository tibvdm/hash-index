//! TODO: First hashing that distributes the k-mers to less buckets

use crate::hash::fnv_1a_hash::Fnv1aHasher32;

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
    pub fn new() -> HashTable {
        HashTable {
            hasher: Fnv1aHasher32::new()
        }
    }

    /// TODO
    pub fn insert(&mut self, kmer: &Kmer) {
        // TODO: keep track of order of kmers in secondary structure
        // Then we can reorder the function table to represent this order
        // Then fp = bucket_index + offset_first_conflict

        let bucket = self.hasher.hash(kmer);

        // The bucket contains the position of the first conflict
        let first_conflict_index = self.buckets[bucket]

        // This is the first hit for our bucket
        // TODO: WILL NEVER BE -1, BECAUSE THIS IS AN UNSIGNED INTEGER 32
        // SOLUTION: Add 1 extra if that will only be executed once, then branching will get rid of this case.
        if first_conflict_index == -1 {
            // Prepare the entry to be pushed (#conflicts in last 19 bits).
            let entry: u64 = kmer.0 | 0x0000200000000000;

            // add the entry to the list
            self.stack.push(vec![entry]);

            // Store the index of this conflict in the bucket
            self.buckets[bucket] = self.stack.length() - 1;
        }

        // This is a conflict for our bucket
        else {
            // prepare entry to be pushed
            let entry: u64 = kmer.0;

            // Retrieve the first conflict
            let first_conflict: u64 = self.stack[first_conflict_index];

            // Retrieve the amount of conflicts for this bucket so far
            let amount_of_conflicts: u64 = first_conflict >> 45;

            // Update the count of conflicts
            amount_of_conflicts += 1;
            self.stack[first_conflict_index][0] = first_conflict & 0x00001FFFFFFFFFFF | amount_of_conflicts << 45;

            // Add the new conflict to the list
            self.stack[first_conflict_index].push(entry);
        }
    }

    /// TODO
    pub fn get(&self, kmer: &Kmer) -> Result<u32> {
        // Calculate the bucket of this k-mer
        let bucket = self.hasher.hash(kmer);

        // Retrieve the position of the first conflict
        let first_conflict_index = self.buckets[bucket]

        // Perform a logarithmic search to find the correct entry
        Ok(log_search(kmer, first_conflict_index))
    }

    /// TODO
    pub fn finish() {
        // TODO: order is OK, because we start with an ordered list of kmers ()
        // TODO: place the 2D vector sequentieel in memory
        // TODO: maybe final changes voor de ftable order list
    }

    fn log_search(&self, kmer: &Kmer, first_conflict_index: u32) -> Result<u32> {
        let lower: u32 = 0;
        let upper: u32 = len(self.stack[first_conflict_index]) - 1;

        while lower < upper {
            let middle: u32 = (lower + upper) / 2;
            if self.stack[first_conflict_index][middle] < kmer.0 {
                lower = middle + 1;
            } else if self.stack[first_conflict_index][middle] > kmer.0 {
                upper = middle - 1;
            } else {
                Ok(first_conflict_index + middle)
            }
        }

        // TODO throw error
        Ok(0)
    }
}
