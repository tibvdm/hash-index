//! Implementation of a ConflictTable

use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

use crate::hash::fnv_1a_hash::Fnv1aHasher32;
use crate::kmer::Kmer;
use crate::errors::Result;

// TODO: Hasher could be static in this case

/// A hash table with uniformly distributed conflicts
#[derive(Serialize, Deserialize)]
pub struct ConflictTable {
    /// The hash function to determine the correct bucket
    #[serde(skip_serializing, skip_deserializing)]
    hasher: Fnv1aHasher32,
    /// The amount of buckets
    amount_of_buckets: usize,
    /// The buckets
    buckets: Vec<u32>,
    /// The storage for all the conflicts
    #[serde(skip_serializing, skip_deserializing)]
    stack: Vec<Vec<u64>>,
    /// TODO: Split in builder and table
    flattened_stack: Vec<u64>
}

impl ConflictTable {
    /// Create a new table with a fixed amount of buckets
    pub fn new(amount_of_buckets: usize) -> ConflictTable {
        ConflictTable {
            hasher: Fnv1aHasher32::new(),
            amount_of_buckets: amount_of_buckets,
            buckets: vec![0; amount_of_buckets],
            stack: vec![],
            flattened_stack: vec![]
        }
    }

    /// Insert a kmer in the conflict table, returns the bucket and number of conflict
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
        let first_conflict_index: usize = self.buckets[bucket] as usize - 1;

        // Perform a logarithmic search to find the correct entry
        self.log_search(kmer, first_conflict_index)
    }

    /// Check if a k-mer is contained in the index
    pub fn contains(&self, kmer: &Kmer) -> bool {
        match self.get(kmer) {
            Ok(_)  => true,
            Err(_) => false
        }
    }

    /// TODO
    pub fn finish(&mut self) {
        for i in 0 .. self.buckets.len() {
            let stack_pointer: usize = self.buckets[i] as usize;
            if stack_pointer != 0 {
                // Get the stack vector of conflicts
                let mut stack_entry: Vec<u64> = self.stack[stack_pointer - 1].clone();

                // Set amount of conflicts in the first item
                let amount_of_conflicts: u64 = stack_entry.len() as u64;
                stack_entry[0] = stack_entry[0] | (amount_of_conflicts << 45);

                // Current length is the index of the first conflict
                let first_conflict_index: u32 = self.flattened_stack.len() as u32;

                // Add all conflicts to the flattened stack
                self.flattened_stack.extend(&stack_entry);

                // The bucket now has to point to the first conflict
                self.buckets[i] = first_conflict_index + 1;
            }
        }

        //println!("{:?}", self.flattened_stack);

        self.stack.clear();
    }

    /// TODO
    pub fn get_amount_of_kmers(&self) -> usize {
        self.flattened_stack.len()
    }

    /// TODO: saving the functional table
    pub fn to_bin(&self, file_path: String) -> Result<()> {
        let mut file = File::create(file_path)?;

        let encoded: Vec<u8> = bincode::serialize(self).unwrap();

        file.write_all(&encoded);

        Ok(())
    }

    /// TODO
    pub fn from_bin(file_path: String) -> ConflictTable {
        let encoded = fs::read(file_path).expect("Unable to read file");

        bincode::deserialize(&encoded[..]).unwrap()
    }

    /// Logarithmic search for conflicts
    fn log_search(&self, kmer: &Kmer, first_conflict_index: usize) -> Result<u32> {
        let mut lower: i32 = first_conflict_index as i32;
        let mut upper: i32 = lower + (self.flattened_stack[first_conflict_index] >> 45) as i32 - 1;

        while lower != upper {
            let middle: i32 = ((lower + upper) as f64 / 2.0).ceil() as i32;
            let stack_item = self.flattened_stack[middle as usize] & 0x00001FFFFFFFFFFF;

            if stack_item > kmer.0 {
                upper = middle - 1;
            } else {
                lower = middle;
            }
        }

        if self.flattened_stack[lower as usize] & 0x00001FFFFFFFFFFF == kmer.0 {
            return Ok(lower as u32);
        }

        bail!("Entry not found")
    }
}
