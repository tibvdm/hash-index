//! TODO

use crate::kmer::Kmer;

/// Struct for a kmer hasher
#[derive(Clone, Copy)]
pub struct KmerHasher();

impl KmerHasher {
    /// Create a new KmerHasher
    pub fn new() -> KmerHasher {
        KmerHasher()
    }

    /// Computes the hash for a k-mer
    pub fn hash(&self, kmer: &Kmer) -> u32 {
        (kmer.0 + (kmer.0 >> 32)) as u32
    }
}
