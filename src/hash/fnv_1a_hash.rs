//! Fowler-Noll-Vo hash module

use crate::kmer::Kmer;

/// Struct for an FNV Hasher
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

    /// TODO
    pub fn hash_optimized(&self, kmer: &Kmer) -> u32 {
        let mut hashed_value: u32 = 0x811c9dc5;

        hashed_value = hashed_value ^ (kmer.0 & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value = hashed_value ^ ((kmer.0 >> 8) & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value = hashed_value ^ ((kmer.0 >> 16) & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value = hashed_value ^ ((kmer.0 >> 24) & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value = hashed_value ^ ((kmer.0 >> 32) & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value = hashed_value ^ ((kmer.0 >> 40) & 0x000000FF) as u32;
        hashed_value = hashed_value.wrapping_mul(0x01000193);

        hashed_value
    }
}
