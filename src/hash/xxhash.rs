//! TODO

use xxhash_rust::const_xxh32::xxh32;

use crate::kmer::Kmer;

/// Struct for an  xxh32hasher
#[derive(Clone, Copy)]
pub struct Xxh32Hasher();

impl Xxh32Hasher {
    /// Create a new Xxh32Hasher
    pub fn new() -> Xxh32Hasher {
        Xxh32Hasher()
    }

    /// Computes the hash for a k-mer
    pub fn hash(&self, kmer: &Kmer) -> u32 {
        xxh32(&kmer.to_be_bytes(), 0xA54F008C)
    }
}
