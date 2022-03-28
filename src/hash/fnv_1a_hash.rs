//! Fowler-Noll-Vo hash module

use crate::kmer::Kmer;

pub struct Fnv1aHasher32();

impl Fnv1aHasher32 {
    /// Create a new Fnv1aHasher32
    pub fn new() -> Fnv1aHasher32 {
        Fnv1aHasher32()
    }

    /// Computes the hash for a k-mer
    pub fn hash(&self, kmer: &Kmer) -> u32 {
        let mut hashed_value = 0x811c9dc5;

        for byte in kmer.to_be_bytes().iter() {
            hashed_value = hashed_value ^ (*byte as u32);
            hashed_value = hashed_value.wrapping_mul(0x01000193);
        }

        hashed_value
    }
}
