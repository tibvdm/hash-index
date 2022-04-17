//! K-mer module

use rand::distributions::{Distribution, Uniform};

/// K-mer represented as an u64
#[derive(Clone, PartialEq, Debug)]
pub struct Kmer(pub u64);

impl Kmer {
    /// Create a new k-mer
    pub fn new(kmer: u64) -> Kmer {
        Kmer(kmer)
    }

    /// k-mer to bytes
    pub fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

impl From<&str> for Kmer {
    /// Create a k-mer from a string
    fn from(s: &str) -> Self {
        let mut u64_repr: u64 = 0;

        for (i, c) in s.chars().rev().enumerate() {
            u64_repr |= match c {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                'I' => 8,
                'J' => 9,
                'K' => 10,
                'L' => 11,
                'M' => 12,
                'N' => 13,
                'O' => 14,
                'P' => 15,
                'Q' => 16,
                'R' => 17,
                'S' => 18,
                'T' => 19,
                'U' => 20,
                'V' => 21,
                'W' => 22,
                'X' => 23,
                'Y' => 24,
                'Z' => 25,
                _   => 26 // TODO: error handle this
            } << (i * 5)
        }

        Kmer(u64_repr)
    }
}

impl From<Kmer> for String {
    /// Create a string from a k-mer
    fn from(kmer: Kmer) -> Self {
        let mut string: String = "".to_string();

        for i in 9 .. 0 {
            string.push(
                match kmer.0 >> (i * 5) & 0x1F {
                    0  => 'A',
                    1  => 'B',
                    2  => 'C',
                    3  => 'D',
                    4  => 'E',
                    5  => 'F',
                    6  => 'G',
                    7  => 'H',
                    8  => 'I',
                    9  => 'J',
                    10 => 'K',
                    11 => 'L',
                    12 => 'M',
                    13 => 'N',
                    14 => 'O',
                    15 => 'P',
                    16 => 'Q',
                    17 => 'R',
                    18 => 'S',
                    19 => 'T',
                    20 => 'U',
                    21 => 'V',
                    22 => 'W',
                    23 => 'X',
                    24 => 'Y',
                    25 => 'Z',
                    _  => ' ' // TODO: error handle this
                }
            );
        }

        string
    }
}

impl From<String> for Kmer {
    /// Create a k-mer from a string
    fn from(s: String) -> Self {
        let mut u64_repr: u64 = 0;

        for (i, c) in s.chars().rev().enumerate() {
            u64_repr |= match c {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                'I' => 8,
                'J' => 9,
                'K' => 10,
                'L' => 11,
                'M' => 12,
                'N' => 13,
                'O' => 14,
                'P' => 15,
                'Q' => 16,
                'R' => 17,
                'S' => 18,
                'T' => 19,
                'U' => 20,
                'V' => 21,
                'W' => 22,
                'X' => 23,
                'Y' => 24,
                'Z' => 25,
                _   => 26 // TODO: error handle this
            } << (i * 5)
        }

        Kmer(u64_repr)
    }
}

/// Generate a list of k-mers
pub fn generate_kmers(n: usize) -> Vec<Kmer> {
    let distribution = Uniform::<u64>::from(0 .. 512_000_000_000);
    let mut rng = rand::thread_rng();

    return (0 .. n).map(|_| Kmer::new(distribution.sample(&mut rng))).collect();
}
