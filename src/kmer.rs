//! K-mer module

/// K-mer represented as an u64
pub struct Kmer(u64);

impl Kmer {
    /// Create a new k-mer
    pub fn new(kmer: u64) -> Kmer {
        Kmer(kmer)
    }
}

impl From<&str> for Kmer {
    /// Create a k-mer from a string
    fn from(s: &str) -> Self {
        let mut u64_repr: u64 = 0;

        for (i, c) in s.chars().enumerate() {
            u64_repr |= match c {
                'A' => 0,
                'C' => 1,
                'D' => 2,
                'E' => 3,
                'F' => 4,
                'G' => 5,
                'H' => 6,
                'I' => 7,
                'K' => 8,
                'L' => 9,
                'M' => 10,
                'N' => 11,
                'P' => 12,
                'Q' => 13,
                'R' => 14,
                'S' => 15,
                'T' => 16,
                'V' => 17,
                'W' => 18,
                'Y' => 19,
                _   => 20 // TODO: error handle this
            } << (i * 5)
        }

        println!("{}", u64_repr);

        Kmer(u64_repr)
    }
}
