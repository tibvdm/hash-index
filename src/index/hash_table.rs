//! TODO: First hashing that distributes the k-mers to less buckets

use crate::hash::fnv_1a_hash::Fnv1aHasher32;

/// TODO
pub struct HashTable {
    /// TODO
    hasher: Fnv1aHasher32,
    /// TODO
    buckets: Vec<u32>
}

impl HashTable {
    /// TODO
    pub fn new() -> HashTable {
        HashTable {
            hasher: Fnv1aHasher32::new()
        }
    }

    /// TODO
    pub fn insert(&self, kmer: &Kmer, lca: u32, fp: u32) {
        // Calculate bucket
        
        // Put this in the correct secondary structure bucket.
    }

    /// TODO
    pub fn finish() {
        // TODO: order the secondary structure
        // TODO: place sequentially in memory
    }
}
